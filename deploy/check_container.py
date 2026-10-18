#!/usr/bin/env python3
"""Exercise a fresh release Compose installation in a disposable Docker project."""
import argparse
import json
import os
from pathlib import Path
import secrets
import subprocess
import tempfile
import urllib.request

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--image', required=True)
args = parser.parse_args()
repo = Path(__file__).resolve().parent.parent
with tempfile.TemporaryDirectory(prefix='nextleaf-container-check-') as temporary:
    root = Path(temporary)
    data = root / 'data'
    data.mkdir()
    subprocess.run(['sudo', 'chown', '10001:10001', str(data)], check=True)
    override = root / 'override.yml'
    override.write_text('services:\n  nextleaf:\n    ports: !override\n      - "127.0.0.1::8787"\n')
    env = {**os.environ, 'TASKMANAGER_IMAGE': args.image, 'NEXTLEAF_DATA_DIR': str(data),
           'NEXTLEAF_MCP_TOKEN': secrets.token_hex(32), 'NEXTLEAF_MCP_HOSTS': '127.0.0.1,localhost'}
    command = ['docker', 'compose', '--project-name', root.name, '--env-file', '/dev/null',
               '-f', str(repo / 'deploy/release/docker-compose.yml'), '-f', str(override)]
    def compose(*arguments, capture=False):
        return subprocess.run(command + list(arguments), env=env, check=True,
                              capture_output=capture, text=True)
    def request(path, body=None, method=None):
        req = urllib.request.Request(base + path, data=json.dumps(body).encode() if body is not None else None,
                                     headers={'Content-Type': 'application/json'}, method=method)
        with urllib.request.urlopen(req, timeout=15) as response:
            return json.load(response)
    try:
        compose('config', '--quiet')
        compose('up', '-d', '--wait', '--wait-timeout', '90', '--pull', 'never')
        base = 'http://' + compose('port', 'nextleaf', '8787', capture=True).stdout.strip()
        health = request('/api/health')
        assert health['status'] == 'ok'
        assert {'tag-management', 'sync-v2'} <= set(health['capabilities'])
        initial = request('/api/sync')
        assert initial['snapshot']['protocol'] == 2
        assert all(not initial['snapshot'][table] for table in ['tasks', 'projects', 'areas', 'tags'])
        with urllib.request.urlopen(base, timeout=15) as response:
            assert b'<html' in response.read()
        person = request('/api/tags', {'name': 'Release contact', 'kind': 'person'})
        task = request('/api/tasks', {'title': 'Release persistence check', 'status': 'waiting'})
        request('/api/tasks/' + task['id'], {'tags': ['Release contact'], 'waiting_for': 'Release contact'}, 'PATCH')
        request('/api/tags/' + person['id'], {'name': 'Renamed contact'}, 'PATCH')
        subprocess.run(['python3', str(repo / 'deploy/check_mcp.py'), '--url', base + '/mcp'], env=env, check=True)
        updated = request('/api/tasks/' + task['id'])
        assert updated['tags'] == ['Renamed contact']
        assert updated['waiting_for'] == 'Renamed contact'
        before = request('/api/sync')
        compose('down')
        compose('up', '-d', '--wait', '--wait-timeout', '90', '--pull', 'never')
        base = 'http://' + compose('port', 'nextleaf', '8787', capture=True).stdout.strip()
        assert request('/api/sync') == before, 'Data or server identity changed after container recreation'
        print('NEXTLEAF_CONTAINER_OK: empty install, web, API, MCP and recreation persistence')
    finally:
        subprocess.run(command + ['down'], env=env, check=False)
        # Only the disposable directory created above belongs to this check.
        subprocess.run(['sudo', 'rm', '-rf', '--', str(data)], check=True)

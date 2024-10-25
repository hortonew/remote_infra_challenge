#!/usr/bin/env python3
import sys

import requests


def check_servers():
    server_file = "/usr/local/nagios/etc/webservers.txt"
    try:
        with open(server_file, "r") as f:
            servers = [line.strip() for line in f.readlines()]

        offline_servers = []
        for server in servers:
            try:
                response = requests.get(f"http://{server}", timeout=5)
                if response.status_code != 200:
                    offline_servers.append(server)
            except requests.exceptions.RequestException:
                offline_servers.append(server)

        if len(offline_servers) == 0:
            print("OK: All servers are online")
            sys.exit(0)
        elif len(offline_servers) == 1:
            print(f"WARNING: Server {offline_servers[0]} is offline")
            sys.exit(1)
        else:
            print(f"CRITICAL: Servers {', '.join(offline_servers)} are offline")
            sys.exit(2)

    except Exception as e:
        print(f"UNKNOWN: Could not read server list. Error: {e}")
        sys.exit(3)


if __name__ == "__main__":
    check_servers()

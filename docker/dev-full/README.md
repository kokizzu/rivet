# Full Development Docker Compose

This Docker Compose is intended for running a full development environment for Rivet.

## Required ports

The following ports need to be open before running Rivet:

- 8080-8082 (Rivet server)
- 9000 (S3)
- 20000-20100 (Rivet client host networking)

## Operation

### Start

Start the cluster with:

```bash
docker-compose -f docker/dev-full/docker-compose.yml up -d
```

This will start the cluster in detached mode. Once complete, visit the dashboard at [http://localhost:8080](http://localhost:8080).

To test creating an actor end-to-end, run:

```bash
./scripts/manual_tests/actors_e2e_js.ts
```

You should see an actor in the actor list in the dashboard.

### Stop

To shut down the Rivet cluster, run:

```bash
docker-compose -f docker/dev-full/docker-compose.yml down
```

When you start the cluster again, your data will still be there.

### Nuke from orbit

To destroy all containers & volumes immediately, run:

```bash
docker-compose -f docker/dev-full/docker-compose.yml down -v -t 0
```

## Development

### Rebuilding

To rebuild all services, run:

```bash
docker-compose -f docker/dev-full/docker-compose.yml up -d --build
```

To rebuild just the server, run:

```bash
docker-compose -f docker/dev-full/docker-compose.yml up -d --build rivet-server
```

### Logs

To fetch logs for a service, run:

```bash
docker-compose -f docker/dev-full/docker-compose.yml logs {name}
```

#### Following

To follow logs, run:

```bash
docker-compose -f docker/dev-full/docker-compose.yml logs -f {name}
```

#### Grep

It's common to use grep (or the more modern
[ripgrep](https://www.google.com/search?q=ripgrep&oq=ripgrep&sourceid=chrome&ie=UTF-8))
to filter logs.

For example, to find all errors in `rivet-server` with the 10 preceding lines, run:

```bash
docker-compose -f docker/dev-full/docker-compose.yml logs rivet-server | grep -B 10 level=error
```

Logs for `rivet-server` and `rivet-client` can also be configured via the environment. See [here](TODO) for more information.

## Troubleshooting

### Have you tried turning it off and on again?

If something gets _really_ screwed up, you can destroy the entire cluster with:

```bash
docker-compose -f docker/dev-full/docker-compose.yml down -v -t 0
```

This will destroy all containers & volumes immediately.

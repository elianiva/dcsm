# WORK IN PROGRESS, NONE OF THIS WORK YET

# Docker Compose Service Manager (dcsm)

A simple tool to manage your docker compose services, treat your service as if an npm package or the likes. Want to add a new service? Simply run `dcsm service add postgre` and you should have a simple postgresql service configured on your docker compose file.

## Motivation

This tool was made because I'm getting tired of having to copy and paste the docker compose configuration everytime I want to spin up a local database for my new project. I also don't like having to look the documentation everytime or scour through StackOverflow everytime I want to configure something.

Hopefully this can serve as a solution to that. I want it so that adding a service is as easy as adding an npm package. Also, when I want to configure something I want to just pick something from a dropdown list or a checkbox while still being capable of overriding them with custom values. The possible values will be taken from common configurations out there or some of the "best practice" (even though I don't like using this word) gathered around.

There will be 2 user interface, one would be the CLI which works similar to NPM with interactive mode and the other one would be the web UI to make it easier when configuring the docker compose.

Yes, I don't like configuring the docker-compose file manually.

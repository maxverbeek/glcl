# GitLab Command Line (interface but who has time to spell out cli)

A tool to hopefully speed up navigating to repositories in the browser. Meant to be used in combination with something
like Rofi or DMenu.

> For example (not implemented yet):
> 
> ```sh
> glcl repos | rofi -dmenu | xargs chromium
> ```
> Should allow you to rapidly (fuzzy) search through all repositories, and pressing enter should open it in your browser
> immediately. This command can be bound to a hotkey in i3 or something so you can open it whenever.

This stuff is inspired by the sluggish frontend of GitLab and the desire to get easier search abilities for open merge
requests.

## Roadmap

Ultimately this repo will contain at least 3 crates: a daemon, a cli, and an ICP protocol crate to bridge them.
Functionality will be strictly divided between the CLI and the daemon:

**Daemon**

 - will query periodically in the background, and contain secrets
 - periodically renew PAT and store it in a secure place (only accessible by root/system user?)
 - Ensure cache is relatively up to date in a smart way (need an algo to determine when to refresh what?)

**CLI** 

 - will interact with daemon over an ICP socket
 - Should be able to give instant responses meant for interactive searches
 - Does not need access to sensitive PAT things

## Progress

 - [x] Query (all) GitLab projects
 - [ ] Implement the example at the top of this readme
 - [ ] Cache API responses
      - SQLite?
      - NoSQL/JSON files on disk?
      - SQLite with JSON cols?
      - im mostly too lazy to implement a database schema for gitlabs API
 - [ ] Search in cache
 - [ ] Split the application into a daemon and a CLI
 - [ ] Query (open) merge requests, and MRs assigned/ready for review
 - [ ] PAT storage & renewal
 - [ ] Smart cache renewal without spamming gitlab/getting rate limitmed too much/using lots of bandwidth

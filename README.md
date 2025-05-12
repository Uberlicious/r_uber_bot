# UberBot (WIP)
Discord bot written in Rust for use in my personal discord server.

## Event Handler
The event handler listens to events on the discord server. This is mainly used to read messages looking for keywords.
Ex. Seeing `gardy time` in a message has a chance to trigger looking up a random gif with the keyword `time`.

### Events
| Trigger | Descripiton |
| -----------|-----------|
| `gardy time` | Look up random gif with the keyword `time` |
| `luxe time` | Look up random gif with either the keyword `bathroom` or `time`|
| REDACTED | Can't tell you | 

## Commands
| Command | Arguments | Description |
|---------|-----------|-------------|
| superhero | none | Get a random superhero pulled from [Superhero API](https://superheroapi.com/) |
| super_duel | @user | Trigger a duel against a random user, whoevers superhero has the highest overall score wins |

## TODO:
More fun slash commands
Custom role assignment to replace role reaction
Custom server management category / role / channels with single command and auto-add to above role assignment

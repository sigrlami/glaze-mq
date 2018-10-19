# GlazeMQ

![GitHub](https://img.shields.io/github/license/mashape/apistatus.svg)
![stability-wip](https://img.shields.io/badge/stability-work_in_progress-lightgrey.svg)

 Multi-protocol messaging broker - WIP - research project

 | Name     | Type    | Description
 |----------|---------|------------------------------------------------
 | `common` | library | shared types and functionality between client and server
 | `server` | app     | exchange broker server
 | `client` | app     | exchange connector
 | `mqtt`   | plugin  | pure Rust implementation of MQTT protocol
 | `ampq`   | plugin  | pure Rust implementation of AMPQ protocol
 | `stomp`  | plugin  | pure Rust implementation of STOMP protocol

## Goals

- [ ] Protocol agnostic server
- [ ] MQTT implementation
- [ ] STOMP implementation
- [ ] AMQP implementation
- [ ] Multiple Queues
- [ ] Multiple Channels per single Queue
- [ ] Crash resistance
- [ ] Retention periods, soft/hard
- [ ] Retention sizes, soft/hard
- [ ] Channel seek by timestamp, id, sender, receiver

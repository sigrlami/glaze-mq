# GlazeMQ - multi-protocol pub/sub broker

 Multi-protocol messaging broker for IoT, 5G-and-Beyond networks with emphasis on scalability.


 | Name     | Type    | Description
 |----------|---------|------------------------------------------------
 | `common` | library | shared types and functionality between client and server
 | `server` | app     | exchange broker server
 | `client` | app     | exchange connector
 | `mqtt`   | plugin  | pure Rust implementation of MQTT protocol
 | `ampq`   | plugin  | pure Rust implementation of AMPQ protocol
 | `stomp`  | plugin  | pure Rust implementation of STOMP protocol


## Rationale

Modern broker exchanges lack perfomance features that allow to scale messaging capabilities according to changes in the traffic of IoT or 5G environments. Contrary to 4G networks that use packet system for burst messaging, exponential increase in IoT devices which is already reaching 1B online devices, sharing data 24/7 between each other and other monitoring system. It's expected that within 5 years we will see further increase in number of device which can bring online up to 5-6 billion devices. In addition to that deployment of 5G networks throughout the world creates absolutely novel environment for messaging which is more distributed and requires more capable system to serve such demand.

## Roadmap

### v0.1

- [x] Protocol agnostic server
- [x] MQTT implementation, v3
- [x] Minimalistic client

### v0.2

- [ ] MQTT add support for new v5
- [ ] AMQP implementation
- [ ] Multiple Queues

### v0.3

- [ ] STOMP implementation
- [ ] Multiple Channels per single Queue
- [ ] Crash resistance

### v0.4

- [ ] Retention periods, soft/hard
- [ ] Retention sizes, soft/hard
- [ ] Channel seek by timestamp, id, sender, receiver

## Sponsorship

This work partially sponsored by [Kelecorix, Inc.](https://kelecorix.com). This infrastructure used for several clients on their IoT cloud solution in Florida and Arizona.

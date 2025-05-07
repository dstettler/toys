Two Operating Modes: Single-entity, brokered client

Single-entity:
- Client *and* server
- On load starts broker on thread 
  - Binds to broker port
  - Awaits new connections
  - On new connection, relay new port and check sender id
    - If id has been added to contacts spawn handshake thread on that port
    - Else, inform user of new connection from sender id xyz, and allow them to start handshake if they wish
- 
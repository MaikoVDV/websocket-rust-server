# Headers for messages sent through the socket
Messages sent through the websocket start with an integer. That specifies the header.

Various kinds of messages can be send through the socket. The header indicates what the message is.

Messages are grouped in 10s, with respect to their function

---
## 0-9: Connection data
### 0: ClientConnect
Sent from the server to all clients when a new client connects.
Contains a single u32, the client's id.

### 1: ClientDisconnect
Sent from the server to all clients when a client disconnects.
Contains that clients id so it can be removed from the world for all clients.
---
## 10-19: State data
### 10: GameStateUpdate
Sent by the server to all clients every tick.
Contains changes in gamestate, like after a client has sent a move input.

### 11: InitialState
Message contains InitialState. Sent once to a client, when they join.
Contains their client_id and the full current state stored in a GameStateUpdate.

---
## 20-29: Input sent by client
### 20: ClientInput
Sent from a client to the server. ClientInput can change the state.
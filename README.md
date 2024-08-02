# P2P-Gossiping-Application
### how to use 
<br>
clone the repositary <br>

```
cd P2P-Gossiping-Application/peer
```
<br>
then <br>
```
cargo bulid
```
<br>
now you can run peer as mentiond in the task and <br>
```
cargo run -- --period 5 --port 8080
```
and the second peer <br>
```
cargo run -- --period 10 --port 8081 --connection 127.0.0.1:8080
```
And so on you can add extra peers.<br>
the output not exactly as mentioned just to make sure for the right connections. check if 

# Packet tracer

This program loads the compiled bytecode and loads it into kernel space. 

## Inspecting the interface

Below are the interfaces:

```
1: lo                # Loopback
2: eno1              # Wired Ethernet
3: wlp0s20f3         # Wi-Fi
```

If we run the following command:

```
ip link show
```

Which can give the following output:

```
1: lo: <LOOPBACK,UP,LOWER_UP> mtu 65536 qdisc noqueue state UNKNOWN mode DEFAULT group default qlen 1000
    link/loopback 00:00:00:00:00:00 brd 00:00:00:00:00:00
2: eno1: <NO-CARRIER,BROADCAST,MULTICAST,UP> mtu 1500 qdisc fq_codel state DOWN mode DEFAULT group default qlen 1000
    link/ether 58:11:22:c5:26:74 brd ff:ff:ff:ff:ff:ff
    altname enp6s0
3: wlp0s20f3: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue state UP mode DORMANT group default qlen 1000
    link/ether 4c:03:4f:53:bb:05 brd ff:ff:ff:ff:ff:ff
4: docker0: <NO-CARRIER,BROADCAST,MULTICAST,UP> mtu 1500 qdisc noqueue state DOWN mode DEFAULT group default 
    link/ether 02:42:18:cd:22:69 brd ff:ff:ff:ff:ff:ff
5: br-07865ba8777b: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue state UP mode DEFAULT group default 
    link/ether 02:42:db:9e:e6:f1 brd ff:ff:ff:ff:ff:ff
13: veth6b8eb51@if12: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue master br-07865ba8777b state UP mode DEFAULT group default 
    link/ether 4a:68:0e:40:ce:a4 brd ff:ff:ff:ff:ff:ff link-netnsid 0
15: veth97fe3ca@if14: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue master br-07865ba8777b state UP mode DEFAULT group default 
    link/ether 7a:e1:4d:b7:18:20 brd ff:ff:ff:ff:ff:ff link-netnsid 1
17: veth83e52b1@if16: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue master br-07865ba8777b state UP mode DEFAULT group default 
    link/ether 62:21:f0:8e:15:3b brd ff:ff:ff:ff:ff:ff link-netnsid 2
```

In our example the WiFi is enabled but the ethernet is not connected and we know what interfaces are available on the computer. We must use one of the interfaces available when running the program.

### Docker interface

You might see that the `docker0` interface in the previous example was `state DOWN`. This is because there are no docker containers running. If a docker container is running the switch gets turned on and we get something like the following:

```
4: docker0: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue state UP mode DEFAULT group default 
    link/ether 02:42:18:cd:22:69 brd ff:ff:ff:ff:ff:ff
```

## Running the program

If we are going to use the `eth0` interface we can just run the `cargo run` command. If we want to interface with different networks, we can do the following:

### Running with Docker network interface

```
RUST_LOG=info cargo run --config 'target."cfg(all())".runner="sudo -E"' --   --iface docker0
```

It must be noted that this interface is for `bridge` networks as opposed to `host` networks. 

### Running with WiFi network interface

```
RUST_LOG=info cargo run --config 'target."cfg(all())".runner="sudo -E"' --   --iface wlp0s20f3
```

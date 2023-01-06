# MMM - Mini mTLS Manager

mmm is a program to create, manage and host an mTLS root authority.

It will allow you to issue certificates for services and manage client side certificates for those services as well.

The idea is to implement something like zero trust networking inside your firewall, every service has knowledge of who it is talking to, both ways.

It also allows you to audit new services being booted on your network as they all have to talk to mmm to get certificates to be able to communicate with anyone else.

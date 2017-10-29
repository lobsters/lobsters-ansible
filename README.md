### Lobste.rs Ansible Playbook

Ansible playbook for lobste.rs.

Lobsters is a technology-focused link aggregation site.

To run:

    $ ansible-playbook -K prod.yml


#### Playbooks

    backup          - TODO: backup site.
    prod            - deploy to production.
    restore         - TODO: restore from backup.


#### Inventory

The following host groups are available:

    backup          - backup, archive, and log server.
    console         - serial console access.  Used to start, stop, and debug
                      a host.
    db              - SQL server.
    dns             - authoritative DNS.
    mx*             - incoming email.
    smtp*           - outgoing email.
    www*            - http over SSL.

  groups marked with an asterisk (*) use public SSL certificates.


The following variables are available:

    backup_server   - database dump, log, static file, and email backup.
    console_server  - serial console (for grub), installer (with live cd),
                      reverse DNS, and SSH key management. 
    db_server       - SQL server.
    dns_server      - authoritative DNS server.
    mx_server       - incoming mail server.
    smtp_server     - outgoing mail server.
    www_server      - http.

When a host group has more than one hostname, the _server variable
contains the authoritative name for the hosted sevice.

This playbook tries not to distinguish between host variables and
group variables.

https://docs.ansible.com/ansible/latest/intro_inventory.html


#### Tags

The following tags can be used to limit tasks in a playbook:

    pkg             - install operating system packages (deb or rpm).
    user            - create or revoke system administrator accounts
                      and public SSH keys.

A role name can be used as a tag.  When given, the tasks in that role
will be run.

https://docs.ansible.com/ansible/latest/playbooks_tags.html


Roles
-----

    mariadb         - SQL database.
    lobsters        - web application.
    nginx           - http proxy and SSL termination.
    sysadm          - accounts and ssh shell acess for system administrators.
    systype         - operating system and host architecture detection.
    postfix         - MX and smtp server.
    unicorn         - Rack/Ruby FastCGI server.

https://docs.ansible.com/ansible/latest/playbooks.html
https://docs.ansible.com/ansible/latest/playbooks_reuse_roles.html


#### Library

The following modules are available:

    systype         - runtime detection of operating system and
                      machine architecture.

https://docs.ansible.com/ansible/latest/dev_guide/developing_modules.html


#### SSH Keys

To use this playbook, you'll need an account in the sysadm role
along with an SSH key pair.

    $ ssh-keygen
    <++> ~/.ssh/config
    Host lobsters.xen.prgmr.com
      IdentityFile ~/.ssh/id_rsa-lobste.rs

    Host lobsters.console.xen.prgmr.com
      User lobsters
      IdentityFile ~/.ssh/id_rsa-lobste.rs
    <-->

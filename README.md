# My Website

Just a simple Django app which is designed to make my life easier.


## Ansible

Setting up a new server should ideally be quite painless. Run the following
from the `ansible` directory.

```bash
$ ansible-playbook sites.yml
```

This should:
- Create a user called "www-data" which has no permissions
- Install Nginx, set it up to proxy requests to "http://localhost:8000/" and
    serve static files from "/var/www/michaelfbryan.com/static/"
- Download the latest version of the website, install all dependencies, then
    start up Gunicorn as a Systemd service called "website.service"

> **Note:** Because ansible uses `python2` by default, you'll need to make sure
> to have that installed on the remote machine.

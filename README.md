# My Website

Just a simple Django app which is designed to make my life easier.


## Ansible

Setting up a new server should ideally be quite painless. Run the following
from the `ansible` directory.

```bash
$ ansible-playbook sites.yml
```

This should:
- Install Nginx, set it up to proxy requests to "http://localhost:8000/" and
    serve static files from "/var/www/michaelfbryan.com/static/"

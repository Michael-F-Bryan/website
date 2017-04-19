# My Website

Just a simple Django app which is designed to make my life easier.


## Features

At the moment there isn't very much to the website...


### Timesheet App

A super basic app for tracking my timesheet entries, writing down what I did on
a particular day, and how many hours I've worked.

It also has a "TimeSlice" function. This gives you some unique, permanent URL
which will show you all the entries for a particular user between two dates.
A possible use case is if you want to send your times to your boss, but don't
want to give them access to your full timesheet or let them make any changes.


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

> **WARNING:** At the moment you'll still need to create a superuser the first
> time you provision a server.
>
> ```
> ./manage.py createsuperuser
> ```
>
> You'll also need to manually copy across the SSL certificates and put them in
> `/etc/nginx/ssl/`. For security reasons they'll be distributed separate from
> repository.


[drf]: http://www.django-rest-framework.org/

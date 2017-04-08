server {
        listen 80 default_server;
        listen [::]:80 default_server;

        server_name _;

        location / {
                proxy_pass http://localhost:8000/;
        }

        location /static/ {
          autoindex on;
          root /home/michael/website/static/;
        }
  }


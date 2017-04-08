server {
        listen 80 default_server;
        listen [::]:80 default_server;

        server_name michaelfbryan.com;

        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Host $server_name;

        location /static {
          alias /home/michael/website/static;
          expires 30d;
        }

        location / {
                proxy_pass http://localhost:8000/;
        }
  }


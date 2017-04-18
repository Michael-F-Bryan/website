server {
    listen         80;
    server_name    {{ domain }};
    return         301 https://$server_name$request_uri;
}

server {
    listen 443 ssl;
    ssl_certificate "/etc/nginx/ssl/bundle.crt";
    ssl_certificate_key "/etc/nginx/ssl/michaelfbryan.com.key";
    add_header Strict-Transport-Security "max-age=31536000"; 

    server_name {{ domain }};

    proxy_set_header Host $host;
    proxy_set_header X-Real-IP $remote_addr;
    proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    proxy_set_header X-Forwarded-Host $server_name;

    location /static/ {
        alias /var/www/{{ domain }}/static/;
        expires 30d;
    }

    location / {
        proxy_pass http://localhost:8000/;
    }
}


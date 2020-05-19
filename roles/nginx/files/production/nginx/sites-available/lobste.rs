upstream lobsters_unicorn_server {
  server unix:/srv/lobste.rs/run/unicorn.sock fail_timeout=0;
}

# lobste.rs http->https redirection
server {
  listen 71.19.148.33:80;
  listen [2605:2700:0:2:a800:ff:fe83:b1e7]:80;
  server_name lobste.rs;
  access_log /var/log/nginx/lobste.rs.access.log main;
  error_log /var/log/nginx/lobste.rs.error.log;
  rewrite ^/(.*)$ https://lobste.rs/$1 permanent;
  keepalive_timeout 0;
}

# www.lobste.rs -> lobste.rs redirection
server {
  listen 71.19.148.33:443 ssl;
  listen [2605:2700:0:2:a800:ff:fe83:b1e7]:443 ssl;
  server_name www.lobste.rs;
  access_log /var/log/nginx/lobste.rs.access.log main;
  error_log /var/log/nginx/lobste.rs.error.log;
  keepalive_timeout 0;
  server_tokens off;

  ssl on;
  ssl_certificate /etc/letsencrypt/live/lobste.rs/fullchain.pem;
  ssl_certificate_key /etc/letsencrypt/live/lobste.rs/privkey.pem;
  ssl_protocols TLSv1.2;
  ssl_ciphers ECDHE-ECDSA-CHACHA20-POLY1305:ECDHE-RSA-CHACHA20-POLY1305:ECDHE-ECDSA-AES128-GCM-SHA256:ECDHE-RSA-AES128-GCM-SHA256:ECDHE-ECDSA-AES256-GCM-SHA384:ECDHE-RSA-AES256-GCM-SHA384:DHE-RSA-AES128-GCM-SHA256:DHE-RSA-AES256-GCM-SHA384:ECDHE-ECDSA-AES128-SHA256:ECDHE-RSA-AES128-SHA256:ECDHE-ECDSA-AES128-SHA:ECDHE-RSA-AES256-SHA384:ECDHE-RSA-AES128-SHA:ECDHE-ECDSA-AES256-SHA384:ECDHE-ECDSA-AES256-SHA:ECDHE-RSA-AES256-SHA:DHE-RSA-AES128-SHA256:DHE-RSA-AES128-SHA:DHE-RSA-AES256-SHA256:DHE-RSA-AES256-SHA:ECDHE-ECDSA-DES-CBC3-SHA:ECDHE-RSA-DES-CBC3-SHA:EDH-RSA-DES-CBC3-SHA:AES128-GCM-SHA256:AES256-GCM-SHA384:AES128-SHA256:AES256-SHA256:AES128-SHA:AES256-SHA:DES-CBC3-SHA:!DSS;
  ssl_prefer_server_ciphers on;
  ssl_dhparam /etc/ssl/dhparams-2048.pem;
  ssl_stapling on;

  rewrite ^/(.*)$ https://lobste.rs/$1 permanent;
  # needs libnginx-mod-http-headers-more-filter available in zesty.
  #more_set_headers 'X-Frame-Options: DENY' 'Strict-Transport-Security: max-age=15552000; includeSubDomains; preload';
}

# main lobste.rs
server {
  listen 71.19.148.33:443 ssl http2 default_server;
  listen [2605:2700:0:2:a800:ff:fe83:b1e7]:443 ssl http2 default_server;
  server_name lobste.rs;

  access_log /var/log/nginx/lobste.rs.access.log main;
  error_log /var/log/nginx/lobste.rs.error.log;

  root /srv/lobste.rs/http/public;

  if (-f $document_root/maintenance.html) {
     return 503;
  }

  ssl on;
  ssl_certificate /etc/letsencrypt/live/lobste.rs/fullchain.pem;
  ssl_certificate_key /etc/letsencrypt/live/lobste.rs/privkey.pem;

  ssl_protocols TLSv1.2;
  ssl_ciphers ECDHE-ECDSA-CHACHA20-POLY1305:ECDHE-RSA-CHACHA20-POLY1305:ECDHE-ECDSA-AES128-GCM-SHA256:ECDHE-RSA-AES128-GCM-SHA256:ECDHE-ECDSA-AES256-GCM-SHA384:ECDHE-RSA-AES256-GCM-SHA384:DHE-RSA-AES128-GCM-SHA256:DHE-RSA-AES256-GCM-SHA384:ECDHE-ECDSA-AES128-SHA256:ECDHE-RSA-AES128-SHA256:ECDHE-ECDSA-AES128-SHA:ECDHE-RSA-AES256-SHA384:ECDHE-RSA-AES128-SHA:ECDHE-ECDSA-AES256-SHA384:ECDHE-ECDSA-AES256-SHA:ECDHE-RSA-AES256-SHA:DHE-RSA-AES128-SHA256:DHE-RSA-AES128-SHA:DHE-RSA-AES256-SHA256:DHE-RSA-AES256-SHA:ECDHE-ECDSA-DES-CBC3-SHA:ECDHE-RSA-DES-CBC3-SHA:EDH-RSA-DES-CBC3-SHA:AES128-GCM-SHA256:AES256-GCM-SHA384:AES128-SHA256:AES256-SHA256:AES128-SHA:AES256-SHA:DES-CBC3-SHA:!DSS;
  ssl_prefer_server_ciphers on;
  ssl_dhparam /etc/ssl/dhparams-2048.pem;
  ssl_stapling on;

  if ($http_user_agent ~* "Brave") { return 400 "Blocked cryptocurrency scam."; }

  location @unicorn {
    proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    proxy_set_header Host $http_host;
    proxy_set_header X-Forwarded-Proto https;
    proxy_redirect off;
    proxy_pass http://lobsters_unicorn_server;
  }

  location ~ ^/assets/ {
    gzip_static on;
    expires     max;
    add_header  Cache-Control public;
    break;
  }

  location ~ ^/avatars/ {
    error_page 418 = @unicorn;
    recursive_error_pages on;

    expires     max;
    add_header  Cache-Control public;
    try_files $uri @unicorn;
    break;
  }

  # file-based full-page caching, bypass if user has cookies
  set $use_file_cache "";
  if ($http_cookie ~* "lobster_trap") {
    set $use_file_cache "${use_file_cache}S";       # S = no session cookie
  }
  if ($http_cookie ~* "tag_filters") {
    set $use_file_cache "${use_file_cache}F";       # F = no filter cookie
  }
  if (-f $document_root/cache/$uri/index.html) {
    set $use_file_cache "${use_file_cache}I";       # I = index file cached
  }
  if ($use_file_cache = "SFI") {
    rewrite (.*) /cache/$1/index.html break;
  }
  if (-f $document_root/cache/$uri.html) {
    set $use_file_cache "${use_file_cache}H";       # H = HTML file cached
  }
  if ($use_file_cache = "SFH") {
    rewrite (.*) /cache/$1.html break;
  }
  if (-f $document_root/cache/$uri) {
    set $use_file_cache "${use_file_cache}O";       # O = other non-extentioned file cached
  }
  if ($use_file_cache = "SFO") {
    rewrite (.*) /cache/$1 break;
  }

  location / {
    proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    proxy_set_header Host $http_host;
    proxy_set_header X-Forwarded-Proto https;
    proxy_redirect off;

    if (!-f $request_filename) {
      proxy_pass http://lobsters_unicorn_server;
      break;
    }
  }

  # needs libnginx-mod-http-headers-more-filter available in zesty.
  #more_set_headers 'X-Frame-Options: DENY' 'Strict-Transport-Security: max-age=15552000; includeSubDomains; preload';

  error_page 500 502 504 /500.html;
  location = /500.html {
    root /srv/lobste.rs/http/public;
  }
}

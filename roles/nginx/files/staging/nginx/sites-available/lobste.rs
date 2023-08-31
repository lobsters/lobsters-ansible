upstream lobsters_puma_server {
  server unix:/srv/lobste.rs/run/puma.sock fail_timeout=0;
}

server {
  listen 80 default_server;
  listen [::]:80 default_server;
  server_name stage01.lobste.rs;
  access_log /srv/lobste.rs/log/lobste.rs.access.log main;
  error_log /srv/lobste.rs/log/lobste.rs.error.log;

  root /srv/lobste.rs/http/public;

  if (-f $document_root/maintenance.html) {
    rewrite  ^(.*)$  /maintenance.html last;
    return 503;
  }

  location @puma {
    proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    proxy_set_header Host $http_host;
    proxy_set_header X-Forwarded-Proto http;
    proxy_redirect off;
    proxy_pass http://lobsters_puma_server;
  }

  location ~ ^/assets/ {
    gzip_static on;
    expires     max;
    add_header  Cache-Control public;
    break;
  }

  location ~ ^/avatars/ {
    error_page 418 = @puma;
    recursive_error_pages on;

    expires     max;
    add_header  Cache-Control public;
    try_files $uri @puma;
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
    proxy_set_header X-Forwarded-Proto http;
    proxy_redirect off;

    if (!-f $request_filename) {
      proxy_pass http://lobsters_puma_server;
      break;
    }
  }

  # needs libnginx-mod-http-headers-more-filter available in zesty.
  #more_set_headers 'X-Frame-Options: DENY' 'Strict-Transport-Security: max-age=15552000; includeSubDomains; preload';

  error_page 502 /502.html;
  error_page 500 504 /500.html;
  location = /500.html {
    root /srv/lobste.rs/http/public;
  }
}

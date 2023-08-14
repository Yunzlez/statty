FROM caddy:2-alpine

COPY dist/ /site/
COPY config/Caddyfile /etc/caddy/Caddyfile

ENV BACKEND_URI=localhost:8080
ENV HOST=localhost

CMD caddy run --config /etc/caddy/Caddyfile

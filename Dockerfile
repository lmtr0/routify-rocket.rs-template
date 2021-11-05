FROM fedora
COPY dist /app
ENTRYPOINT /app/server
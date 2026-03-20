FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y bc && rm -rf /var/lib/apt/lists/*
COPY index.html /index.html
COPY calculate.sh /calculate.sh
RUN chmod +x /calculate.sh

FROM ubuntu:22.04
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
      python3 \
      python3-pip
COPY requirements.txt /requirements.txt
RUN pip install -r /requirements.txt

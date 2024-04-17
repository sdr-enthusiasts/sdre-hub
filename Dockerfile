FROM ghcr.io/sdr-enthusiasts/docker-baseimage:base

ENV AR_LISTEN_UDP_ACARS=5550 \
    AR_LISTEN_TCP_ACARS=5550 \
    AR_LISTEN_UDP_VDLM2=5555 \
    AR_LISTEN_TCP_VDLM2=5555 \
    AR_LISTEN_UDP_HFDL=5556 \
    AR_LISTEN_TCP_HFDL=5556 \
    AR_LISTEN_ZMQ_ACARS=35550 \
    AR_LISTEN_ZMQ_VDLM2=35555 \
    AR_LISTEN_ZMQ_HFDL=35556 \
    AR_SERVE_TCP_ACARS=15550 \
    AR_SERVE_TCP_VDLM2=15555 \
    AR_SERVE_TCP_HFDL=15556 \
    AR_SERVE_ZMQ_ACARS=45550 \
    AR_SERVE_ZMQ_VDLM2=45555 \
    AR_SERVE_ZMQ_HFDL=45556 \
    AR_ADD_PROXY_ID=true

SHELL ["/bin/bash", "-o", "pipefail", "-c"]
COPY ./rootfs /
COPY ./bin/sdre-hub.armv7/sdre-hub /opt/sdre-hub.armv7
COPY ./bin/sdre-hub.arm64/sdre-hub /opt/sdre-hub.arm64
COPY ./bin/sdre-hub.amd64/sdre-hub /opt/sdre-hub.amd64

# hadolint ignore=DL3008,DL3003,SC1091
RUN set -x && \
    KEPT_PACKAGES=() && \
    TEMP_PACKAGES=() && \
    KEPT_PACKAGES+=(libzmq5) && \
    apt-get update && \
    apt-get install -y --no-install-recommends \
    "${KEPT_PACKAGES[@]}" \
    "${TEMP_PACKAGES[@]}"\
    && \
    # ensure binaries are executable
    chmod -v a+x \
    /opt/sdre-hub.armv7 \
    /opt/sdre-hub.arm64 \
    /opt/sdre-hub.amd64 \
    && \
    # remove foreign architecture binaries
    /rename_current_arch_binary.sh && \
    rm -fv \
    /opt/sdre-hub.* \
    && \
    # clean up
    apt-get remove -y "${TEMP_PACKAGES[@]}" && \
    apt-get autoremove -y && \
    rm -rf /src/* /tmp/* /var/lib/apt/lists/* && \
    # test
    /opt/sdre-hub --version

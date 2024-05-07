FROM ghcr.io/sdr-enthusiasts/docker-baseimage:base

SHELL ["/bin/bash", "-o", "pipefail", "-c"]
COPY ./rootfs /
COPY ./bin/sdre-hub.armv7/sdre-hub /opt/sdre-hub.armv7
COPY ./bin/sdre-hub.arm64/sdre-hub /opt/sdre-hub.arm64
COPY ./bin/sdre-hub.amd64/sdre-hub /opt/sdre-hub.amd64
COPY ./bin/sh-frontend /opt/sh-frontend
ENV SH_DATA_PATH="/opt/sdre-hub-data"
# hadolint ignore=DL3008,DL3003,SC1091
RUN set -x && \
    KEPT_PACKAGES=() && \
    TEMP_PACKAGES=() && \
    KEPT_PACKAGES+=(libzmq5) && \
    KEPT_PACKAGES+=(nginx-light) && \
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

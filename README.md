# lxc-idmap

lxc-idmap is a utility that generates the configuration needed to map the provided UIDs and GIDs between the host and a lxc container.

## Usage

```
Usage: lxc-idmap [OPTIONS] [MAPPINGS]...

Arguments:
  [MAPPINGS]...
          Mappings for both uid and gid

Options:
  -u, --uid <UID_ONLY_MAPPINGS>
          Mappings only for uid

  -g, --gid <GID_ONLY_MAPPINGS>
          Mappings only for gid

      --debug
          Print debug messages in stderr

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

Mappings can be provided in the following format:

    CT_ID_START[-CT_ID_END][:HOST_ID_START[-HOST_ID_END]]

CT_ID_START and CT_ID_END are the UID/GID range start and end from the container's view.
If CT_ID_END is not provided it defaults to the same value as CT_ID_START.
HOST_ID_START and HOST_ID_END are the UID/GID range start and end from the host's view.
If HOST_ID_END is not provided it defaults to HOST_ID_START + CT_ID_END - CT_ID_START

All range bounds are inclusive.
```
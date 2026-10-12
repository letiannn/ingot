CLI Reference
=============

::

    ingot -m <PATH>... [OPTIONS]

Required
--------

====================================  ==========================================
Option                                Description
====================================  ==========================================
``-m, --model <PATH>``                Path to TOML data model file(s) or
                                      directory of TOML files. Repeatable —
                                      multiple ``-m`` flags merge all inputs
                                      into a single data model.
====================================  ==========================================

Output & Target
---------------

======================  ============  ==========================================
Option                  Default       Description
======================  ============  ==========================================
``-o, --output <DIR>``  ``generated/``  Output directory for generated C code
``-t, --target <T>``    ``linux``     Target platform (see below)
``--no-events``         off           Disable event callback generation
======================  ============  ==========================================

Key Filtering
-------------

=====================================  ==========================================
Option                                 Description
=====================================  ==========================================
``--include-list <FILE>``              YAML list of keys to include (whitelist);
                                       all others are excluded
``--exclude-list <FILE>``              YAML list of keys to exclude (blacklist);
                                       all others are included
``--persistent-keys <FILE>``           YAML list of keys that should be marked
                                       persistent
``--property-override-list <FILE>``    YAML map of per-key property overrides
                                       (``KEY: {default_value: val}``)
=====================================  ==========================================

``--include-list`` and ``--exclude-list`` are mutually exclusive.

Key names in filter lists are matched with underscore-normalized comparison,
so both ``STATEOFCHARGE`` and ``STATE_OF_CHARGE`` will match the same key.

Variant Overrides
-----------------

=========================  ==========================================
Option                     Description
=========================  ==========================================
``--variant <NAME>``       Product variant for per-variant default
                           overrides (e.g. ``peabodyv0``,
                           ``omnidrive_v2``). Applies variant-specific
                           defaults from ``[classes.keys.defaults]``
                           and merges variant enum values from
                           ``[enums.*.variants.*]``.
=========================  ==========================================

Verbosity
---------

=========================  ============  ==========================================
Option                     Default       Description
=========================  ============  ==========================================
``-v`` / ``-vv`` / ``-vvv``  warn       Verbosity: info / debug / trace
=========================  ============  ==========================================


Targets
-------

===============  ==================  ====================  ==============================
Value            Platform            Mutex                 Notes
===============  ==================  ====================  ==============================
``baremetal``    Bare-metal          None                  32-bit, 4-byte alignment
``freertos``     FreeRTOS            FreeRTOS semaphore    ``xSemaphoreTake``/``Give``
``rtthread``     RT-Thread           RT-Thread mutex       ``rt_mutex_take``/``release``
``linux``        64-bit Linux        pthread mutex         ``PTHREAD_MUTEX_INITIALIZER``
===============  ==================  ====================  ==============================

Thread-safe keys are protected by the target's mutex mechanism. Keys without
``thread_safe = true`` bypass locking entirely.


Examples
--------

Generate from a single model file::

    ingot -m device.toml -o src/dm/ -t baremetal

Merge a directory of namespace TOML files::

    ingot -m models/ -o gen/udm -t linux

Merge files from multiple locations::

    ingot -m common/base.toml -m project/overrides.toml -o gen/

Use include list and persistent keys::

    ingot -m models/ --include-list conf/include.yml \
        --persistent-keys conf/persist.yml -o gen/

Apply per-key property overrides::

    ingot -m models/ --property-override-list conf/overrides.yml -o gen/

Generate for a specific product variant::

    ingot -m models/ --variant peabodyv0 -o gen/
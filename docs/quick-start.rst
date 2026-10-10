Quick Start
===========

1. Write a data model in TOML:

.. code-block:: toml

    [meta]
    id = "my_device"
    version = "1.0.0"

    [[classes]]
    id = "config"

        [[classes.keys]]
        id = "brightness"
        type = "uint8"
        default = 100

2. Generate C code:

::

    ingot --model my_device.toml --output generated/

3. Include the generated files in your build system and use the API:

.. code-block:: c

    #include "dm.h"
    #include "dm_key_tbl.h"

    DataModel_Initialize(NULL);

    DM_VAL_T val;
    val.u8val = 75;
    DataModel_SetIntegralTypeByKey(DM_KEY_MY_DEVICE_CONFIG_BRIGHTNESS, val);

    DM_VAL_T out = DataModel_GetIntegralTypeByKey(DM_KEY_MY_DEVICE_CONFIG_BRIGHTNESS);
    // out.u8val == 75
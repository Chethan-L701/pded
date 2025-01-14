import tensorflow as tf

# Set tensorflow to make use of the available deveice
gpus = tf.config.list_physical_devices('GPU')

if gpus:
    tf.config.set_visible_devices(gpus[0], 'GPU');
    tf.config.experimental.set_memory_growth(gpus[0], True)

    details = tf.config.experimental.get_device_details(gpus[0])
    pci_bus_id = details.get('device_name', 'Unknown')
    print(f"Using GPU : {pci_bus_id}")


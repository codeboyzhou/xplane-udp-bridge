import time

from termcolor import colored

from dataref import DataRefReader
from udp import UdpClient

if __name__ == "__main__":
    # Create UDP client
    client = UdpClient("127.0.0.1", 49000)

    # Create DataRefReader
    dataref_reader = DataRefReader(client)

    while True:
        # Read dataref value examples
        data_refs = [
            ("sim/cockpit2/controls/parking_brake_ratio", "float"),
            ("sim/cockpit2/engine/actuators/throttle_ratio", "float"),
            ("sim/cockpit2/engine/actuators/eng_master", "[int]"),
            ("sim/cockpit2/electrical/battery_on", "[int]"),
        ]

        for data_ref, type_str in data_refs:
            value = dataref_reader.read(data_ref, type_str)
            if value is not None:
                print(colored(f"Dataref {data_ref} successfully read as {type_str}: {value}", "green"))

        # Sleep for a short duration to avoid overloading the server
        time.sleep(3)

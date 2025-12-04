import time

from src.dataref import DataRefReader
from src.udp import UdpClient

if __name__ == "__main__":
    # Create UDP client
    client = UdpClient("127.0.0.1", 49000)

    # Create DataRefReader
    dataref_reader = DataRefReader(client)

    while True:
        # Read dataref value examples
        data_refs = ["sim/cockpit2/controls/parking_brake_ratio"]
        for data_ref in data_refs:
            dataref_reader.read_as_float(data_ref)
        # Sleep for a short duration to avoid overloading the server
        time.sleep(3)

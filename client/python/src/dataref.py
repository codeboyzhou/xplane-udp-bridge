from src.udpclient import UdpClient


class DataRefReader:
    """DataRefReader class for XPlane UDP bridge plugin."""

    def __init__(self, client: UdpClient):
        """
        Initialize DataRefReader.

        Args:
            client (UdpClient): UDP client instance.
        """
        self.client = client

    def read_as_float(self, data_ref: str) -> float | None:
        """
        Read a dataref as float.

        Args:
            data_ref (str): dataref name (e.g., "sim/cockpit2/controls/parking_brake_ratio")

        Returns:
            float | None: dataref value or None on timeout
        """
        data = f"dataref|read|float|{data_ref}"
        print(f"➡️ Sending dataref read request: {data}")
        response = self.client.send_and_recv(data.encode())
        if response:
            response_body = response.decode().strip()
            print(f"⬅️ Received dataref read response body: {response_body}")
            value = float(response_body.split("|")[-1])
            print(f"✅ Dataref {data_ref} successfully read as float: {value}")
            return value
        else:
            print(f"❌ Dataref {data_ref} read failed: no response from server")
            return None

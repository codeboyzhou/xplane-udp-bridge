"""
DataRef Reader module for XPlane UDP bridge plugin.

This module provides functionality to read data references (datarefs) from XPlane through the UDP bridge plugin.
Datarefs are variables in XPlane that can be read or written to control or monitor various aspects of the simulation.

Example:
    >>> from src.udp import UdpClient
    >>> from src.dataref import DataRefReader
    >>> client = UdpClient("127.0.0.1", 49000)
    >>> reader = DataRefReader(client)
    >>> parking_brake = reader.read_as_float("sim/cockpit2/controls/parking_brake_ratio")
    >>> print(f"Parking brake ratio: {parking_brake}")
"""

from src.udp import UdpClient


class DataRefReader:
    """
    DataRefReader class for XPlane UDP bridge plugin.

    This class provides methods to read data references from XPlane through the UDP bridge plugin.
    It handles the formatting of requests and parsing of responses for different data types.

    Attributes:
        client (UdpClient): UDP client instance used for communication with the XPlane plugin.
    """

    def __init__(self, client: UdpClient):
        """
        Initialize DataRefReader with a UDP client.

        Args:
            client (UdpClient): UDP client instance for communication with the XPlane plugin.

        Example:
            >>> from src.udp import UdpClient
            >>> client = UdpClient("127.0.0.1", 49000)
            >>> reader = DataRefReader(client)
        """
        self.client = client

    def read_as_float(self, data_ref: str) -> float | None:
        """
        Read a data reference as a float value.

        Sends a request to read the specified data reference from XPlane and returns the value as a float.
        The data reference is a string that identifies a specific variable in XPlane.

        Args:
            data_ref (str): Data reference name (e.g., "sim/cockpit2/controls/parking_brake_ratio").
                           These are the standard XPlane dataref identifiers.

        Returns:
            float | None: The float value of the data reference, or None if the request fails
                         due to timeout or other communication issues.

        Example:
            >>> reader = DataRefReader(client)
            >>> altitude = reader.read_as_float("sim/cockpit2/gauges/indicators/altitude_ft_pilot")
            >>> if altitude is not None:
            ...     print(f"Current altitude: {altitude} feet")
            ... else:
            ...     print("Failed to read altitude")
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

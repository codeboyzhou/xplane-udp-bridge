"""
DataRef Reader module for XPlane UDP bridge plugin.

This module provides functionality to read data references (datarefs) from XPlane through the UDP bridge plugin.
Datarefs are variables in XPlane that can be read or written to control or monitor various aspects of the simulation.

Example:
    >>> from udp import UdpClient
    >>> from dataref import DataRefReader
    >>> client = UdpClient("127.0.0.1", 49000)
    >>> reader = DataRefReader(client)
    >>> parking_brake = reader.read("sim/cockpit2/controls/parking_brake_ratio", "float")
    >>> print(f"Parking brake ratio: {parking_brake}")
"""

from uuid import uuid4

from termcolor import colored

from udp import UdpClient


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
            >>> from udp import UdpClient
            >>> client = UdpClient("127.0.0.1", 49000)
            >>> reader = DataRefReader(client)
        """
        self.client = client

    def read(self, data_ref: str, type_str: str) -> str | None:
        """
        Read a data reference as a specified value type.

        Sends a request to read the specified data reference from XPlane and returns the value as a string.
        The data reference is a string that identifies a specific variable in XPlane.

        Args:
            data_ref (str): Data reference name (e.g., "sim/cockpit2/controls/parking_brake_ratio").
                           These are the standard XPlane dataref identifiers.
            type_str (str): Type of the data reference (e.g., "int", "float", "[int]", "[float]").

        Returns:
            str | None: The string value of the data reference, or None if the request fails
                       due to timeout or other communication issues.

        Example:
            >>> reader = DataRefReader(client)
            >>> altitude = reader.read("sim/cockpit2/gauges/indicators/altitude_ft_pilot", "float")
            >>> if altitude is not None:
            ...     print(f"Current altitude: {altitude} feet")
            ... else:
            ...     print("Failed to read altitude")
        """
        data = f"{uuid4().hex}|dataref|read|{type_str}|{data_ref}"
        print("=" * 100)
        print(colored(f"Sending dataref read request: {data}", "cyan"))
        response = self.client.send_and_recv(data.encode())
        if response:
            response_body = response.decode().strip()
            print(colored(f"Received dataref read response body: {response_body}", "yellow"))
            value = response_body.split("|")[-1]
            return value
        else:
            print(colored(f"Dataref {data_ref} read failed: no response from server", "red"))
            return None

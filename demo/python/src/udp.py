"""
UDP Client module for XPlane UDP bridge plugin.

This module provides a UDP client implementation for communicating with the XPlane UDP bridge plugin.
It allows sending data requests and receiving responses from the XPlane simulation environment.

Example:
    >>> client = UdpClient("127.0.0.1", 49000)
    >>> response = client.send_and_recv(b"dataref|read|float|sim/cockpit2/controls/parking_brake_ratio")
    >>> print(response)
    b'dataref|response|float|0.0'
"""

import socket

from termcolor import colored


class UdpClient:
    """
    UDP Client for XPlane UDP bridge plugin.

    This class handles the communication with the XPlane UDP bridge plugin using the UDP protocol.
    It provides methods to send data and receive responses from the server.

    Attributes:
        server_addr (tuple): Tuple containing the server IP and port.
        socket (socket.socket): UDP socket for communication with the server.
    """

    def __init__(self, host: str, port: int, timeout_secs: float = 3):
        """
        Initialize UDP Client for XPlane UDP bridge plugin.

        Creates a UDP socket and configures it to communicate with the specified server.

        Args:
            host (str): Server IP address (e.g., "127.0.0.1" for localhost).
            port (int): Server port number (e.g., 49000 for XPlane UDP bridge).
            timeout_secs (float, optional): Socket timeout in seconds. Defaults to 3.

        Example:
            >>> client = UdpClient("127.0.0.1", 49000, 10)
            >>> print(client.server_addr)
            ('127.0.0.1', 49000)
        """
        print("=" * 100)
        print(colored(f"Creating UDP client to server {host}:{port} with timeout {timeout_secs} seconds", "cyan"))
        self.server_addr = (host, port)
        self.socket = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        self.socket.settimeout(timeout_secs)
        print(colored("Created UDP client successfully", "green"))

    def send_and_recv(self, data: bytes) -> bytes | None:
        """
        Send data to the server and wait for a response.

        Sends the provided data to the configured server address and waits for a response.
        Handles timeout and general exceptions gracefully.

        Args:
            data (bytes): Data to send to the server.

        Returns:
            bytes | None: Response bytes from the server, or None if a timeout or error occurs.

        Example:
            >>> client = UdpClient("127.0.0.1", 49000)
            >>> response = client.send_and_recv(b"test_data")
            >>> if response:
            ...     print("Received:", response)
            ... else:
            ...     print("No response received")
        """
        try:
            self.socket.sendto(data, self.server_addr)
        except Exception as e:
            print(colored(f"UDP error while sending data: {e}", "red"))
            return None

        try:
            response, _ = self.socket.recvfrom(2048)
            return response
        except Exception as e:
            print(colored(f"UDP error while receiving data: {e}", "red"))
            return None

import socket


class UdpClient:
    """UDP Client for XPlane UDP bridge plugin."""

    def __init__(self, host: str, port: int, timeout_secs: float = 30):
        """
        Initialize UDP Client for XPlane UDP bridge plugin.

        Args:
            host (str): server IP (e.g., "127.0.0.1")
            port (int): server port (e.g., 49000)
            timeout_secs (float, optional): socket timeout seconds. Defaults to 30.
        """
        print(f"🔌 Connecting to {host}:{port} with timeout {timeout_secs} seconds")
        self.server_addr = (host, port)
        self.socket = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        self.socket.settimeout(timeout_secs)
        print("✅ Connected successfully via UDP protocol")

    def send_and_recv(self, data: bytes) -> bytes | None:
        """
        Send bytes and wait for response.

        Args:
            data (bytes): data to send

        Returns:
            bytes | None: response bytes or None on timeout
        """
        try:
            self.socket.sendto(data, self.server_addr)
            response, _ = self.socket.recvfrom(2048)
            return response
        except TimeoutError:
            print(f"⏱ UDP request timed out after {self.socket.gettimeout()} seconds")
            return None
        except Exception as e:
            print(f"❌ UDP error: {e}")
            return None

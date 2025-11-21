import socket
import time


class UdpClient:
    """UDP Client for XPlane UDP bridge plugin."""

    def __init__(self, host: str, port: int, timeout: float = 3.0):
        """
        Initialize UDP Client for XPlane UDP bridge plugin.

        Args:
            host (str): server IP (e.g., "127.0.0.1")
            port (int): server port (e.g., 49000)
            timeout (float, optional): socket timeout seconds. Defaults to 3.0.
        """
        print(f"🔌 connecting to {host}:{port} with timeout {timeout} seconds")
        self.server_addr = (host, port)
        self.socket = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        self.socket.settimeout(timeout)
        print("✅ connected successfully via UDP protocol")

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

    def close(self):
        """Close the socket."""
        self.socket.close()


if __name__ == "__main__":
    client = UdpClient("127.0.0.1", 49000)

    try:
        for i in range(5):
            msg = f"hello message {i}"

            print(f"➡️ sending message {i}: {msg}")
            resp = client.send_and_recv(msg.encode())

            if resp:
                print(f"⬅️ received message {i}: {resp}")
            else:
                print(f"⚠️ no response from server for message {i}")

            # sleep for 1 second between requests to avoid overwhelming the server
            time.sleep(1)
    finally:
        client.close()
        print("UDP client closed")

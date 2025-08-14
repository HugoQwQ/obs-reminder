<script lang="ts">
  import { SvelteToast, toast } from "@zerodevx/svelte-toast";
  import { onMount } from "svelte";

  let isConnected = false;
  let websocket: WebSocket | null = null;
  const WEBSOCKET_URL = "ws://localhost:7981";

  interface ToastMessage {
    title: string;
    content: string;
    color_1: string;
    color_2: string;
    text_color: string;
    duration: number;
  }

  interface WebSocketMessage {
    type: string;
    data: ToastMessage;
  }

  function connectWebSocket() {
    try {
      websocket = new WebSocket(WEBSOCKET_URL);

      websocket.onopen = () => {
        console.log("WebSocket connected");
        toast.push("客户端连接成功");
        isConnected = true;
      };

      websocket.onmessage = (event) => {
        try {
          const message: WebSocketMessage = JSON.parse(event.data);
          if (message.type === "toast") {
            showToast(message.data);
          }
        } catch (error) {
          console.error("Error parsing WebSocket message:", error);
        }
      };

      websocket.onclose = () => {
        console.log("WebSocket disconnected");
        if (isConnected) {
          toast.push("客户端已断开连接");
          console.log("WebSocket disconnected #1");
          isConnected = false;
        }
        setTimeout(connectWebSocket, 2000);
      };

      websocket.onerror = (error) => {
        console.error("WebSocket error:", error);
      };
    } catch (error) {
      console.error("Failed to connect WebSocket:", error);
      setTimeout(connectWebSocket, 2000);
    }
  }

  function showToast(data: ToastMessage) {
    toast.push(`${data.title}<br><strong>${data.content}</strong>`, {
      theme: {
        "--toastBackground": data.color_1,
        "--toastBarBackground": data.color_2,
        "--toastColor": data.text_color,
      },
      duration: data.duration,
      dismissable: false,
    });
  }

  onMount(() => {
    connectWebSocket();

    return () => {
      if (websocket) {
        websocket.close();
      }
    };
  });
</script>

<main>
  <SvelteToast />
</main>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    background: transparent;
    font-family:
      -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
  }

  main {
    width: 100vw;
    height: 100vh;
    background: transparent;
  }
</style>

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
    play_sound: boolean;
    sound_url?: string;
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
        toast.push("OBS Remainer<br><strong>客户端连接成功</strong>");
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
          toast.push("OBS Remainer<br><strong>客户端连接已断开</strong>");
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

  function playNotificationSound(soundUrl?: string) {
    try {
      if (!soundUrl) {
        console.log('No sound URL provided');
        return;
      }

      console.log('Attempting to play sound:', soundUrl);

      // 创建音频对象，使用用户选择的音频文件
      const audio = new Audio(soundUrl);
      audio.volume = 0.7; // 设置音量为70%

      // 添加事件监听器来调试
      audio.addEventListener('loadstart', () => {
        console.log('Audio loading started');
      });

      audio.addEventListener('canplay', () => {
        console.log('Audio can start playing');
      });

      audio.addEventListener('error', (e) => {
        console.error('Audio error event:', e);
        console.error('Audio error details:', audio.error);
      });

      audio.addEventListener('loadeddata', () => {
        console.log('Audio data loaded');
      });

      // 尝试播放
      const playPromise = audio.play();

      if (playPromise !== undefined) {
        playPromise
          .then(() => {
            console.log('Audio played successfully');
          })
          .catch(error => {
            console.error('Audio play failed:', error);
            console.error('Error name:', error.name);
            console.error('Error message:', error.message);
          });
      }
    } catch (error) {
      console.error('Audio creation failed:', error);
    }
  }

  function showToast(data: ToastMessage) {
    if (data.play_sound && data.sound_url) {
      playNotificationSound(data.sound_url);
    }

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

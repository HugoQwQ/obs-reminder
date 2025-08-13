<script lang="ts">
	import { SvelteToast, toast } from '@zerodevx/svelte-toast';
	import { onMount } from 'svelte';

	let websocket: WebSocket | null = null;
	let isConnected = false;
	const WEBSOCKET_URL = 'ws://localhost:7981';

	interface ToastMessage {
		title: string;
		content: string;
		color_1: string;
		color_2: string;
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
				isConnected = true;
				console.log('WebSocket connected');
			};

			websocket.onmessage = (event) => {
				try {
					const message: WebSocketMessage = JSON.parse(event.data);
					if (message.type === 'toast') {
						showToast(message.data);
					}
				} catch (error) {
					console.error('Error parsing WebSocket message:', error);
				}
			};

			websocket.onclose = () => {
				isConnected = false;
				console.log('WebSocket disconnected');
				setTimeout(connectWebSocket, 5000);
			};

			websocket.onerror = (error) => {
				console.error('WebSocket error:', error);
			};
		} catch (error) {
			console.error('Failed to connect WebSocket:', error);
			setTimeout(connectWebSocket, 5000);
		}
	}

	function showToast(data: ToastMessage) {
		toast.push(`<strong>${data.title}</strong><br>${data.content}`, {
			theme: {
				'--toastBackground': data.color_1,
				'--toastBarBackground': data.color_2
			},
			duration: data.duration
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

	<div class="status" class:connected={isConnected}>
		{isConnected ? 'Connected' : 'Disconnected'}
	</div>
</main>

<style>
	:global(body) {
		margin: 0;
		padding: 0;
		background: transparent;
		font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
	}

	main {
		width: 100vw;
		height: 100vh;
		background: transparent;
	}

	.status {
		position: fixed;
		top: 10px;
		right: 10px;
		padding: 4px 8px;
		border-radius: 4px;
		font-size: 12px;
		background: rgba(255, 0, 0, 0.8);
		color: white;
		opacity: 0.7;
	}

	.status.connected {
		background: rgba(0, 255, 0, 0.8);
	}
</style>

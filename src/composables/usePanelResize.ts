// src/composables/usePanelResize.ts
import { ref, onMounted, onUnmounted, Ref } from "vue";

export function usePanelResize(
  containerRef: Ref<HTMLElement | null>,
  direction: "horizontal" | "vertical" = "horizontal"
) {
  const panels = ref<number[]>([300, 600, 300]);
  const isDragging = ref(false);
  const dragIndex = ref(-1);

  const onMouseDown = (index: number, e: MouseEvent) => {
    isDragging.value = true;
    dragIndex.value = index;
    e.preventDefault();
  };

  const onMouseMove = (e: MouseEvent) => {
    if (!isDragging.value || !containerRef.value) return;

    const rect = containerRef.value.getBoundingClientRect();
    const pos = direction === "horizontal" ? e.clientX - rect.left : e.clientY - rect.top;

    const total = panels.value.reduce((a, b) => a + b, 0);
    const before = panels.value.slice(0, dragIndex.value).reduce((a, b) => a + b, 0);

    const minWidth = 200;
    const newWidth = Math.max(minWidth, Math.min(pos - before, total - before - minWidth * (panels.value.length - dragIndex.value)));

    panels.value[dragIndex.value] = newWidth;
  };

  const onMouseUp = () => {
    isDragging.value = false;
    dragIndex.value = -1;
  };

  onMounted(() => {
    window.addEventListener("mousemove", onMouseMove);
    window.addEventListener("mouseup", onMouseUp);
  });

  onUnmounted(() => {
    window.removeEventListener("mousemove", onMouseMove);
    window.removeEventListener("mouseup", onMouseUp);
  });

  return { panels, isDragging, onMouseDown };
}
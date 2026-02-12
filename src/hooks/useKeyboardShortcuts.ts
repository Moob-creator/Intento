import { useEffect, useRef } from 'react';

type ShortcutHandler = () => void;

interface ShortcutConfig {
  key: string;
  metaKey?: boolean;
  ctrlKey?: boolean;
  shiftKey?: boolean;
  handler: ShortcutHandler;
}

/**
 * Custom hook for global keyboard shortcuts
 * Handles cross-platform support (Cmd on macOS, Ctrl on Windows/Linux)
 */
export function useKeyboardShortcuts(shortcuts: ShortcutConfig[]) {
  const handlersRef = useRef(shortcuts);

  useEffect(() => {
    handlersRef.current = shortcuts;
  }, [shortcuts]);

  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      const isMac = navigator.platform.toUpperCase().indexOf('MAC') >= 0;

      for (const shortcut of handlersRef.current) {
        const matchesKey = event.key.toLowerCase() === shortcut.key.toLowerCase();
        const matchesMeta = shortcut.metaKey ? (isMac ? event.metaKey : event.ctrlKey) : true;
        const matchesCtrl = shortcut.ctrlKey ? event.ctrlKey : true;
        const matchesShift = shortcut.shiftKey ? event.shiftKey : true;

        if (matchesKey && matchesMeta && matchesCtrl && matchesShift) {
          event.preventDefault();
          shortcut.handler();
          break;
        }
      }
    };

    window.addEventListener('keydown', handleKeyDown);

    return () => {
      window.removeEventListener('keydown', handleKeyDown);
    };
  }, []);
}

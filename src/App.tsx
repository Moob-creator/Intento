import { useState, useEffect, useMemo } from 'react';
import { Sparkles, Send, X } from 'lucide-react';
import { useTaskStore } from './store/taskStore';
import { TopBar } from './components/TopBar';
import { Sidebar } from './components/Sidebar';
import { TaskList } from './components/TaskList';
import { TaskDetailPanel } from './components/TaskDetailPanel';
import { TaskConfirmDialog } from './components/TaskConfirmDialog';
import { TaskOperationsConfirmDialog } from './components/TaskOperationsConfirmDialog';
import { CommandPalette } from './components/CommandPalette';
import { StatisticsPanel } from './components/StatisticsPanel';
import { SettingsPanel } from './components/SettingsPanel';
import { SummaryPanel } from './components/SummaryPanel';  // ✨ Phase 5
import { useKeyboardShortcuts } from './hooks/useKeyboardShortcuts';
import { invoke } from '@tauri-apps/api/core';
import type { Task, TaskStatus, ParsedTask, ImageParseResult, TaskOperation } from './types/task';
import './App.css';

function App() {
  const {
    tasks,
    selectedTask,
    isLoading,
    error,
    loadTasks,
    createTask,
    updateTask,
    deleteTask,
    selectTask,
    clearError,
  } = useTaskStore();

  // Panel states
  const [commandPaletteOpen, setCommandPaletteOpen] = useState(false);
  const [statisticsPanelOpen, setStatisticsPanelOpen] = useState(false);
  const [settingsPanelOpen, setSettingsPanelOpen] = useState(false);
  const [summaryPanelOpen, setSummaryPanelOpen] = useState(false);  // ✨ Phase 5
  const [initialViewMode, setInitialViewMode] = useState<'current' | 'history'>('current');  // ✨ Phase 5.4

  // Filter state
  const [statusFilter, setStatusFilter] = useState<'all' | TaskStatus>('all');
  const [selectedTag, setSelectedTag] = useState<string | null>(null);
  const [sidebarCollapsed, setSidebarCollapsed] = useState(false);

  // AI text input state
  const [textInput, setTextInput] = useState('');
  const [textInputVisible, setTextInputVisible] = useState(false);
  const [showConfirmDialog, setShowConfirmDialog] = useState(false);
  const [parsedTask, setParsedTask] = useState<ParsedTask | null>(null);
  const [isParsing, setIsParsing] = useState(false);
  const [parseError, setParseError] = useState<string | null>(null);

  // Image input state
  const [pastedImage, setPastedImage] = useState<string | null>(null);
  const [imageType, setImageType] = useState<string>('image/png');

  // Multi-operation confirm state
  const [showOperationsDialog, setShowOperationsDialog] = useState(false);
  const [imageParseResult, setImageParseResult] = useState<ImageParseResult | null>(null);

  // Load tasks on mount
  useEffect(() => {
    loadTasks();
  }, [loadTasks]);

  // Global keyboard shortcuts
  useKeyboardShortcuts([
    {
      key: 'k',
      metaKey: true,
      handler: () => setCommandPaletteOpen((prev) => !prev),
    },
    {
      key: 'n',
      metaKey: true,
      handler: handleNewTask,
    },
    {
      key: '/',
      metaKey: true,
      handler: handleOpenTextInput,
    },
    {
      key: ',',
      metaKey: true,
      handler: () => setSettingsPanelOpen(true),
    },
    {
      key: 'r',
      metaKey: true,
      handler: () => setSummaryPanelOpen((prev) => !prev),  // ✨ Phase 5: ⌘R for Summary
    },
    {
      key: 'Escape',
      handler: () => {
        setCommandPaletteOpen(false);
        setStatisticsPanelOpen(false);
        setSettingsPanelOpen(false);
        if (textInputVisible) {
          handleCloseTextInput();
        } else if (selectedTask) {
          handleCancelEdit();
        }
      },
    },
  ]);

  // Handle creating a new task
  function handleNewTask() {
    const newTask: Task = {
      title: '',
      description: '',
      status: 'todo',
      priority: 'medium',
      created_at: Date.now() / 1000,
      updated_at: Date.now() / 1000,
    };
    selectTask(newTask);
  }

  // Handle saving task (create or update)
  const handleSaveTask = async (updates: Partial<Task>) => {
    if (selectedTask?.id) {
      // Update existing task
      await updateTask(selectedTask.id, updates);
    } else {
      // Create new task
      await createTask({
        title: updates.title!,
        description: updates.description,
        status: updates.status || 'todo',
        priority: updates.priority || 'medium',
        deadline: updates.deadline,
        tags: updates.tags,
      } as Omit<Task, 'id' | 'created_at' | 'updated_at'>);
    }
  };

  // Handle status change from card
  const handleStatusChange = async (taskId: number, newStatus: TaskStatus) => {
    await updateTask(taskId, {
      status: newStatus,
      completed_at: newStatus === 'done' ? Math.floor(Date.now() / 1000) : undefined,
    });
  };

  // Handle edit from card
  const handleEditFromCard = (task: Task) => {
    selectTask(task);
  };

  // ✨ Phase 5: Handle generate summary from sidebar context menu
  const handleGenerateSummary = (tag: string) => {
    setSelectedTag(tag);
    setSummaryPanelOpen(true);
  };

  // ✨ Phase 5: Handle view summary history from sidebar context menu
  const handleViewSummaryHistory = (tag: string) => {
    setSelectedTag(tag);
    setInitialViewMode('history');
    setSummaryPanelOpen(true);
  };

  // Handle deleting task
  const handleDeleteTask = async (id: number) => {
    await deleteTask(id);
  };

  // Handle cancel editing
  const handleCancelEdit = () => {
    selectTask(null);
  };

  // Handle opening text input for AI parsing
  function handleOpenTextInput() {
    setTextInputVisible(true);
    setTextInput('');
    setParseError(null);
    setPastedImage(null); // Clear any pasted image
  }

  // Handle closing text input
  const handleCloseTextInput = () => {
    setTextInputVisible(false);
    setTextInput('');
    setParseError(null);
    setPastedImage(null);
  };

  // Handle parsing text or image input
  const handleParseText = async () => {
    // If there's an image, parse it instead of text
    if (pastedImage) {
      await handleParseImage();
      return;
    }

    const trimmed = textInput.trim();
    if (!trimmed) {
      setParseError('Please enter a task description or paste an image');
      return;
    }

    setIsParsing(true);
    setParseError(null);
    setTextInputVisible(false);

    try {
      const result = await invoke<ParsedTask>('parse_text_input', { text: trimmed });
      setParsedTask(result);
      setShowConfirmDialog(true);
    } catch (error) {
      console.error('Failed to parse text input:', error);
      setParseError(
        error instanceof Error ? error.message : 'Failed to parse your input. Please try again.'
      );
      setTextInputVisible(true);
    } finally {
      setIsParsing(false);
    }
  };

  // Handle parsing image input
  const handleParseImage = async () => {
    if (!pastedImage) {
      setParseError('No image to parse');
      return;
    }

    setIsParsing(true);
    setParseError(null);
    setTextInputVisible(false);

    try {
      // Extract base64 data from data URI
      const base64Data = pastedImage.split(',')[1];

      console.log('Parsing image with operations:', {
        imageType,
        base64Length: base64Data?.length || 0,
        dataUriPrefix: pastedImage.substring(0, 50),
      });

      // Use new parse_image_for_operations command
      const result = await invoke<ImageParseResult>('parse_image_for_operations', {
        imageBase64: base64Data,
        imageType: imageType,
        useAllTools: true, // Enable all operations (create, update, complete, delete, etc.)
      });

      console.log('Image parse result:', result);

      // Show operations confirmation dialog
      setImageParseResult(result);
      setShowOperationsDialog(true);
      setPastedImage(null); // Clear image after successful parse
    } catch (error) {
      console.error('Failed to parse image:', error);
      const errorMessage = error instanceof Error ? error.message : String(error);
      console.error('Error details:', errorMessage);

      setParseError(`图片分析失败: ${errorMessage}`);
      setTextInputVisible(true);
    } finally {
      setIsParsing(false);
    }
  };

  // Handle paste event for images
  const handlePaste = (e: React.ClipboardEvent) => {
    const items = e.clipboardData?.items;
    if (!items) return;

    for (let i = 0; i < items.length; i++) {
      const item = items[i];
      if (item.type.startsWith('image/')) {
        e.preventDefault();
        const file = item.getAsFile();
        if (file) {
          const reader = new FileReader();
          reader.onload = (event) => {
            const dataUrl = event.target?.result as string;
            setPastedImage(dataUrl);
            setImageType(file.type);
            setTextInput(''); // Clear text when image is pasted
          };
          reader.readAsDataURL(file);
        }
        break;
      }
    }
  };

  // Handle drag and drop for images
  const handleDragOver = (e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
  };

  const handleDrop = (e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();

    const files = e.dataTransfer?.files;
    if (!files || files.length === 0) return;

    const file = files[0];
    if (file.type.startsWith('image/')) {
      const reader = new FileReader();
      reader.onload = (event) => {
        const dataUrl = event.target?.result as string;
        setPastedImage(dataUrl);
        setImageType(file.type);
        setTextInput('');
      };
      reader.readAsDataURL(file);
    } else {
      setParseError('Please drop an image file (PNG, JPG, etc.)');
    }
  };

  // Remove pasted image
  const handleRemoveImage = () => {
    setPastedImage(null);
  };

  // Handle confirming parsed task
  const handleConfirmTask = async (task: ParsedTask) => {
    setParseError(null);
    try {
      await createTask({
        title: task.title,
        description: task.description,
        status: 'todo',
        priority: task.priority || 'medium',
        deadline: task.deadline ? new Date(task.deadline).getTime() / 1000 : undefined,
        tags: task.tags,
      } as Omit<Task, 'id' | 'created_at' | 'updated_at'>);

      setShowConfirmDialog(false);
      setParsedTask(null);
      setTextInput('');
    } catch (error) {
      console.error('Failed to create task:', error);
      setParseError(
        error instanceof Error ? error.message : 'Failed to create task. Please try again.'
      );
    }
  };

  // Handle canceling confirm dialog
  const handleCancelConfirm = () => {
    setShowConfirmDialog(false);
    setParsedTask(null);
    setParseError(null);
  };

  // Handle confirming operations from image
  const handleConfirmOperations = async (operations: TaskOperation[]) => {
    setParseError(null);
    let successCount = 0;
    let errorCount = 0;
    const errors: string[] = [];

    try {
      // Execute each operation sequentially
      for (const operation of operations) {
        try {
          await executeTaskOperation(operation);
          successCount++;
        } catch (error) {
          errorCount++;
          const errorMsg =
            error instanceof Error ? error.message : 'Unknown error';
          errors.push(`${getOperationLabel(operation)}: ${errorMsg}`);
          console.error(`Failed to execute operation:`, operation, error);
        }
      }

      // Show results
      if (successCount > 0) {
        console.log(`成功执行 ${successCount} 个操作`);
        await loadTasks(); // Reload tasks to show updates
      }

      if (errorCount > 0) {
        setParseError(
          `部分操作失败 (${errorCount}/${operations.length}):\n${errors.join('\n')}`
        );
      } else {
        // All successful, close dialog
        setShowOperationsDialog(false);
        setImageParseResult(null);
      }
    } catch (error) {
      console.error('Error during operations execution:', error);
      setParseError(
        error instanceof Error ? error.message : 'Failed to execute operations'
      );
    }
  };

  // Execute a single task operation
  const executeTaskOperation = async (operation: TaskOperation): Promise<void> => {
    switch (operation.type) {
      case 'Create':
        await createTask({
          title: operation.data.title,
          description: operation.data.description,
          status: 'todo',
          priority: operation.data.priority || 'medium',
          deadline: operation.data.deadline
            ? new Date(operation.data.deadline).getTime() / 1000
            : undefined,
          tags: operation.data.tags,
        } as Omit<Task, 'id' | 'created_at' | 'updated_at'>);
        break;

      case 'Update': {
        // Find task by identifier (title match for now)
        const task = tasks.find((t) =>
          t.title.toLowerCase().includes(operation.data.task_identifier.toLowerCase())
        );
        if (!task?.id) {
          throw new Error(`Task not found: ${operation.data.task_identifier}`);
        }
        await updateTask(task.id, {
          title: operation.data.title,
          description: operation.data.description,
          priority: operation.data.priority,
          deadline: operation.data.deadline
            ? new Date(operation.data.deadline).getTime() / 1000
            : undefined,
          tags: operation.data.tags,
        });
        break;
      }

      case 'Complete': {
        const task = tasks.find((t) =>
          t.title.toLowerCase().includes(operation.data.task_identifier.toLowerCase())
        );
        if (!task?.id) {
          throw new Error(`Task not found: ${operation.data.task_identifier}`);
        }
        await updateTask(task.id, {
          status: 'done',
          completed_at: Math.floor(Date.now() / 1000),
        });
        break;
      }

      case 'Delete': {
        const task = tasks.find((t) =>
          t.title.toLowerCase().includes(operation.data.task_identifier.toLowerCase())
        );
        if (!task?.id) {
          throw new Error(`Task not found: ${operation.data.task_identifier}`);
        }
        await deleteTask(task.id);
        break;
      }

      case 'BatchComplete': {
        for (const identifier of operation.data.task_identifiers) {
          const task = tasks.find((t) =>
            t.title.toLowerCase().includes(identifier.toLowerCase())
          );
          if (task?.id) {
            await updateTask(task.id, {
              status: 'done',
              completed_at: Math.floor(Date.now() / 1000),
            });
          }
        }
        break;
      }

      case 'SetStatus': {
        const task = tasks.find((t) =>
          t.title.toLowerCase().includes(operation.data.task_identifier.toLowerCase())
        );
        if (!task?.id) {
          throw new Error(`Task not found: ${operation.data.task_identifier}`);
        }
        await updateTask(task.id, {
          status: operation.data.status,
          completed_at:
            operation.data.status === 'done' ? Math.floor(Date.now() / 1000) : undefined,
        });
        break;
      }

      default:
        throw new Error(`Unknown operation type`);
    }
  };

  // Get operation label for error messages
  const getOperationLabel = (operation: TaskOperation): string => {
    switch (operation.type) {
      case 'Create':
        return `创建任务: ${operation.data.title}`;
      case 'Update':
        return `更新任务: ${operation.data.task_identifier}`;
      case 'Complete':
        return `完成任务: ${operation.data.task_identifier}`;
      case 'Delete':
        return `删除任务: ${operation.data.task_identifier}`;
      case 'BatchComplete':
        return `批量完成 ${operation.data.task_identifiers.length} 个任务`;
      case 'SetStatus':
        return `设置状态: ${operation.data.task_identifier}`;
      default:
        return '未知操作';
    }
  };

  // Handle canceling operations dialog
  const handleCancelOperations = () => {
    setShowOperationsDialog(false);
    setImageParseResult(null);
    setParseError(null);
  };

  // Handle test notification
  const handleTestNotification = async () => {
    try {
      await invoke('test_notification');
      console.log('Test notification sent successfully');
      // Show a temporary success message
      const successMessage = document.createElement('div');
      successMessage.className = 'fixed top-20 right-6 bg-green-50 border border-green-200 text-green-800 px-4 py-3 rounded-lg shadow-lg z-50 animate-slide-up';
      successMessage.textContent = '✓ Test notification sent! Check your notification center.';
      document.body.appendChild(successMessage);
      setTimeout(() => {
        successMessage.remove();
      }, 3000);
    } catch (error) {
      console.error('Failed to send test notification:', error);
      // Show error message
      const errorMessage = document.createElement('div');
      errorMessage.className = 'fixed top-20 right-6 bg-rose-50 border border-rose-200 text-rose-800 px-4 py-3 rounded-lg shadow-lg z-50 animate-slide-up';
      errorMessage.textContent = `✗ Failed to send notification: ${error}`;
      document.body.appendChild(errorMessage);
      setTimeout(() => {
        errorMessage.remove();
      }, 5000);
    }
  };

  // Sort tasks: High priority first, then by status, then by creation date
  const sortedTasks = useMemo(() => {
    // Apply filters
    let filtered = tasks;

    // Filter by status
    if (statusFilter !== 'all') {
      filtered = filtered.filter(task => task.status === statusFilter);
    }

    // Filter by tag
    if (selectedTag) {
      filtered = filtered.filter(task => task.tags?.includes(selectedTag));
    }

    return [...filtered].sort((a, b) => {
      // Priority order: high > medium > low
      const priorityOrder = { high: 0, medium: 1, low: 2 };
      if (priorityOrder[a.priority] !== priorityOrder[b.priority]) {
        return priorityOrder[a.priority] - priorityOrder[b.priority];
      }

      // Status order: doing > todo > done
      const statusOrder = { doing: 0, todo: 1, done: 2 };
      if (statusOrder[a.status] !== statusOrder[b.status]) {
        return statusOrder[a.status] - statusOrder[b.status];
      }

      // Creation date: newest first
      return b.created_at - a.created_at;
    });
  }, [tasks, statusFilter, selectedTag]);

  return (
    <div className="relative flex flex-col h-screen w-full bg-background overflow-hidden">
      {/* Top bar */}
      <TopBar
        onSearchClick={() => setCommandPaletteOpen(true)}
        onAIClick={handleOpenTextInput}
        onSettingsClick={() => setSettingsPanelOpen(true)}
        onSummaryClick={() => setSummaryPanelOpen(true)}  // ✨ Phase 5
        onSidebarToggle={() => setSidebarCollapsed(!sidebarCollapsed)}
        sidebarCollapsed={sidebarCollapsed}
      />

      {/* Main content */}
      <main className="flex-1 flex overflow-hidden">
        {/* Sidebar */}
        <Sidebar
          tasks={tasks}
          selectedTag={selectedTag}
          onTagSelect={setSelectedTag}
          isCollapsed={sidebarCollapsed}
          onToggleCollapse={() => setSidebarCollapsed(!sidebarCollapsed)}
          onGenerateSummary={handleGenerateSummary}
          onViewSummaryHistory={handleViewSummaryHistory}
        />

        {/* Task list section */}
        <div className="flex-1 flex flex-col px-8 py-6 overflow-y-auto min-w-0">
          {/* Error message */}
          {error && (
            <div className="mb-4 p-4 rounded-xl bg-red-50 border border-red-200">
              <p className="text-red-800 text-sm">
                {error}
                <button onClick={clearError} className="ml-2 underline font-medium">
                  Dismiss
                </button>
              </p>
            </div>
          )}

          {/* Quick actions bar */}
          <div className="flex items-center justify-between mb-6">
            <div>
              <h2 className="text-2xl font-bold text-neutral-dark">
                {selectedTag ? (
                  <span className="flex items-center gap-2">
                    <span className="text-primary">#</span>
                    {selectedTag}
                  </span>
                ) : (
                  <>
                    {statusFilter === 'all' && 'All Tasks'}
                    {statusFilter === 'todo' && 'To Do'}
                    {statusFilter === 'doing' && 'Doing'}
                    {statusFilter === 'done' && 'Done'}
                  </>
                )}
              </h2>
              <p className="text-sm text-neutral-dark/60 mt-1">
                {sortedTasks.length} {sortedTasks.length === 1 ? 'task' : 'tasks'}
                {selectedTag && ` • ${selectedTag}`}
              </p>
            </div>

            {/* Quick filter buttons */}
            <div className="flex gap-2">
              <button
                onClick={() => setStatusFilter('all')}
                className={`px-4 py-2 rounded-lg text-sm font-medium transition-all duration-200 ${
                  statusFilter === 'all'
                    ? 'bg-neutral-dark text-white shadow-md'
                    : 'bg-neutral-light/40 text-neutral-dark/70 hover:bg-neutral-light/60'
                }`}
              >
                All
              </button>
              <button
                onClick={() => setStatusFilter('todo')}
                className={`px-4 py-2 rounded-lg text-sm font-medium transition-all duration-200 ${
                  statusFilter === 'todo'
                    ? 'bg-blue-500 text-white shadow-md'
                    : 'bg-blue-50 text-blue-600 hover:bg-blue-100'
                }`}
              >
                To Do
              </button>
              <button
                onClick={() => setStatusFilter('doing')}
                className={`px-4 py-2 rounded-lg text-sm font-medium transition-all duration-200 ${
                  statusFilter === 'doing'
                    ? 'bg-amber-500 text-white shadow-md'
                    : 'bg-amber-50 text-amber-600 hover:bg-amber-100'
                }`}
              >
                Doing
              </button>
              <button
                onClick={() => setStatusFilter('done')}
                className={`px-4 py-2 rounded-lg text-sm font-medium transition-all duration-200 ${
                  statusFilter === 'done'
                    ? 'bg-emerald-500 text-white shadow-md'
                    : 'bg-emerald-50 text-emerald-600 hover:bg-emerald-100'
                }`}
              >
                Done
              </button>
            </div>
          </div>

          {/* Task list */}
          <TaskList
            tasks={sortedTasks}
            selectedTaskId={selectedTask?.id ?? null}
            onTaskClick={selectTask}
            onStatusChange={handleStatusChange}
            onEdit={handleEditFromCard}
            onDelete={handleDeleteTask}
            isLoading={isLoading}
          />
        </div>

        {/* Task detail panel - slide from right */}
        {selectedTask && (
          <TaskDetailPanel
            task={selectedTask}
            onSave={handleSaveTask}
            onDelete={handleDeleteTask}
            onCancel={handleCancelEdit}
          />
        )}
      </main>

      {/* AI Input Modal - centered modal window */}
      {textInputVisible && (
        <>
          {/* Backdrop */}
          <div
            className="fixed inset-0 bg-black/40 backdrop-blur-sm z-40 animate-fade-in"
            onClick={handleCloseTextInput}
          />

          {/* Modal content */}
          <div className="fixed inset-0 z-50 flex items-center justify-center p-8 pointer-events-none">
            <div className="w-full max-w-3xl bg-white rounded-3xl shadow-2xl pointer-events-auto animate-scale-up">
              <div className="p-8">
                <div className="flex items-center justify-between mb-6">
                  <div className="flex items-center gap-3">
                    <Sparkles size={24} className="text-amber-500" />
                    <span className="text-lg font-semibold text-neutral-dark">
                      {pastedImage ? '分析图片中的任务信息' : '描述任务或粘贴图片'}
                    </span>
                  </div>
                  <button
                    onClick={handleCloseTextInput}
                    className="p-2.5 text-neutral-dark/40 hover:text-neutral-dark hover:bg-neutral-light/40 rounded-xl transition-all duration-200"
                    aria-label="Close"
                  >
                    <X size={22} />
                  </button>
                </div>

                {parseError && (
                  <div className="mb-5 p-4 rounded-xl bg-rose-50 border border-rose-200">
                    <p className="text-sm text-rose-800 leading-relaxed">{parseError}</p>
                  </div>
                )}

                {/* Image preview */}
                {pastedImage && (
                  <div className="mb-5 relative">
                    <img
                      src={pastedImage}
                      alt="Pasted screenshot"
                      className="w-full max-h-72 object-contain rounded-xl border-2 border-amber-200"
                    />
                    <button
                      onClick={handleRemoveImage}
                      className="absolute top-3 right-3 p-2 bg-white/95 hover:bg-white rounded-xl shadow-md transition-all duration-200"
                      aria-label="Remove image"
                    >
                      <X size={18} className="text-neutral-dark/60" />
                    </button>
                  </div>
                )}

                <div
                  className="flex gap-4"
                  onDragOver={handleDragOver}
                  onDrop={handleDrop}
                >
                  <textarea
                    value={textInput}
                    onChange={(e) => setTextInput(e.target.value)}
                    onPaste={handlePaste}
                    onKeyDown={(e) => {
                      if (e.key === 'Enter' && (e.metaKey || e.ctrlKey)) {
                        handleParseText();
                      }
                    }}
                    placeholder={
                      pastedImage
                        ? '图片已粘贴，可添加补充说明...'
                        : '例如："周五前完成季度报告，高优先级，工作项目"\n\n或者粘贴/拖拽图片（⌘V 或拖拽文件）'
                    }
                    rows={pastedImage ? 3 : 5}
                    autoFocus
                    disabled={isParsing}
                    className="flex-1 px-5 py-4 rounded-2xl bg-amber-50/50 text-neutral-dark
                             placeholder:text-neutral-dark/40 placeholder:select-text border-2 border-amber-100 resize-none
                             focus:outline-none focus:border-amber-400/50 focus:bg-amber-50/80
                             transition-all duration-200 text-base leading-relaxed"
                  />
                  <button
                    onClick={handleParseText}
                    disabled={isParsing || (!textInput.trim() && !pastedImage)}
                    className="self-end h-14 px-7 rounded-2xl bg-gradient-to-r from-orange-400 to-rose-400
                             text-white font-bold text-base transition-all duration-200
                             hover:from-orange-500 hover:to-rose-500 hover:shadow-lg hover:scale-105
                             disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:scale-100
                             flex items-center gap-2.5"
                  >
                    {isParsing ? (
                      <>
                        <svg
                          className="animate-spin h-5 w-5"
                          xmlns="http://www.w3.org/2000/svg"
                          fill="none"
                          viewBox="0 0 24 24"
                        >
                          <circle
                            className="opacity-25"
                            cx="12"
                            cy="12"
                            r="10"
                            stroke="currentColor"
                            strokeWidth="4"
                          />
                          <path
                            className="opacity-75"
                            fill="currentColor"
                            d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                          />
                        </svg>
                      </>
                    ) : (
                      <>
                        <Send size={20} />
                      </>
                    )}
                  </button>
                </div>
                <p className="text-sm text-neutral-dark/50 mt-3.5 leading-relaxed select-text">
                  {pastedImage
                    ? '点击发送分析图片 • 或添加文字补充'
                    : '⌘+Enter 提交 • ⌘+V 粘贴图片 • 拖拽图片到此处'}
                </p>
              </div>
            </div>
          </div>
        </>
      )}

      {/* Task Confirm Dialog */}
      <TaskConfirmDialog
        isOpen={showConfirmDialog}
        parsedTask={parsedTask}
        onConfirm={handleConfirmTask}
        onCancel={handleCancelConfirm}
        isLoading={isLoading}
        error={parseError}
      />

      {/* Task Operations Confirm Dialog */}
      <TaskOperationsConfirmDialog
        isOpen={showOperationsDialog}
        parseResult={imageParseResult}
        onConfirm={handleConfirmOperations}
        onCancel={handleCancelOperations}
        isLoading={isLoading}
        error={parseError}
      />

      {/* Command Palette */}
      <CommandPalette
        isOpen={commandPaletteOpen}
        onClose={() => setCommandPaletteOpen(false)}
        tasks={tasks}
        onAIAdd={handleOpenTextInput}
        onNewTask={handleNewTask}
        onShowStats={() => setStatisticsPanelOpen(true)}
        onShowSettings={() => setSettingsPanelOpen(true)}
        onTestNotification={handleTestNotification}
        onTaskSelect={selectTask}
      />

      {/* Statistics Panel */}
      <StatisticsPanel
        isOpen={statisticsPanelOpen}
        onClose={() => setStatisticsPanelOpen(false)}
        tasks={tasks}
      />

      {/* Settings Panel */}
      <SettingsPanel isOpen={settingsPanelOpen} onClose={() => setSettingsPanelOpen(false)} />

      {/* ✨ Phase 5: Summary Panel */}
      <SummaryPanel
        isOpen={summaryPanelOpen}
        onClose={() => {
          setSummaryPanelOpen(false);
          setInitialViewMode('current');
        }}
        selectedTag={selectedTag}
        initialViewMode={initialViewMode}
      />
    </div>
  );
}

export default App;

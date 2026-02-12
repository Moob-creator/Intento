import { useState, useEffect } from 'react';
import { X, CheckCircle2, Edit, Trash2, List, Settings } from 'lucide-react';
import type { TaskOperation, ImageParseResult } from '../types/task';

interface TaskOperationsConfirmDialogProps {
  isOpen: boolean;
  parseResult: ImageParseResult | null;
  onConfirm: (operations: TaskOperation[]) => void;
  onCancel: () => void;
  isLoading?: boolean;
  error?: string | null;
}

export function TaskOperationsConfirmDialog({
  isOpen,
  parseResult,
  onConfirm,
  onCancel,
  isLoading = false,
  error = null,
}: TaskOperationsConfirmDialogProps) {
  const [selectedOperations, setSelectedOperations] = useState<Set<number>>(new Set());

  // Select all operations by default when dialog opens
  useEffect(() => {
    if (isOpen && parseResult?.operations) {
      setSelectedOperations(new Set(parseResult.operations.map((_, i) => i)));
    }
  }, [isOpen, parseResult]);

  if (!isOpen || !parseResult) {
    return null;
  }

  const handleToggleOperation = (index: number) => {
    setSelectedOperations((prev) => {
      const next = new Set(prev);
      if (next.has(index)) {
        next.delete(index);
      } else {
        next.add(index);
      }
      return next;
    });
  };

  const handleConfirm = () => {
    const selected = parseResult.operations.filter((_, i) => selectedOperations.has(i));
    if (selected.length > 0) {
      onConfirm(selected);
    }
  };

  const getOperationIcon = (operation: TaskOperation) => {
    switch (operation.type) {
      case 'Create':
        return <List size={18} className="text-emerald-500" />;
      case 'Update':
        return <Edit size={18} className="text-amber-500" />;
      case 'Complete':
        return <CheckCircle2 size={18} className="text-blue-500" />;
      case 'Delete':
        return <Trash2 size={18} className="text-rose-500" />;
      case 'BatchComplete':
        return <CheckCircle2 size={18} className="text-blue-500" />;
      case 'SetStatus':
        return <Settings size={18} className="text-violet-500" />;
      default:
        return <List size={18} className="text-neutral-dark/60" />;
    }
  };

  const getOperationLabel = (operation: TaskOperation): string => {
    switch (operation.type) {
      case 'Create':
        return '创建任务';
      case 'Update':
        return '更新任务';
      case 'Complete':
        return '完成任务';
      case 'Delete':
        return '删除任务';
      case 'BatchComplete':
        return '批量完成';
      case 'SetStatus':
        return '设置状态';
      default:
        return '未知操作';
    }
  };

  const getOperationDescription = (operation: TaskOperation): string => {
    switch (operation.type) {
      case 'Create':
        return operation.data.title;
      case 'Update':
        return `${operation.data.task_identifier}${operation.data.title ? ` → ${operation.data.title}` : ''}`;
      case 'Complete':
        return operation.data.task_identifier;
      case 'Delete':
        return operation.data.task_identifier;
      case 'BatchComplete':
        return `${operation.data.task_identifiers.length} 个任务`;
      case 'SetStatus':
        return `${operation.data.task_identifier} → ${operation.data.status}`;
      default:
        return '';
    }
  };

  return (
    <>
      {/* Backdrop */}
      <div
        className="fixed inset-0 bg-black/40 backdrop-blur-sm z-40 animate-fade-in"
        onClick={onCancel}
      />

      {/* Dialog content */}
      <div className="fixed inset-0 z-50 flex items-center justify-center p-6 pointer-events-none">
        <div className="w-full max-w-2xl max-h-[80vh] bg-white rounded-2xl shadow-2xl pointer-events-auto animate-scale-up flex flex-col">
          {/* Header */}
          <div className="p-6 border-b border-neutral-light/40">
            <div className="flex items-center justify-between">
              <div>
                <h2 className="text-lg font-semibold text-neutral-dark">
                  确认操作 ({parseResult.operations.length})
                </h2>
                {parseResult.image_description && (
                  <p className="text-sm text-neutral-dark/60 mt-1">
                    {parseResult.image_description}
                  </p>
                )}
                {parseResult.confidence > 0 && parseResult.confidence < 1 && (
                  <p className="text-xs text-neutral-dark/40 mt-1">
                    识别置信度: {(parseResult.confidence * 100).toFixed(0)}%
                  </p>
                )}
              </div>
              <button
                onClick={onCancel}
                className="p-2 text-neutral-dark/40 hover:text-neutral-dark hover:bg-neutral-light/40 rounded-lg transition-all duration-200"
                aria-label="Close"
              >
                <X size={20} />
              </button>
            </div>

            {/* Warnings */}
            {parseResult.warnings.length > 0 && (
              <div className="mt-4 p-3 rounded-lg bg-amber-50 border border-amber-200">
                <p className="text-sm font-medium text-amber-800 mb-1">注意:</p>
                {parseResult.warnings.map((warning, i) => (
                  <p key={i} className="text-xs text-amber-700">
                    • {warning}
                  </p>
                ))}
              </div>
            )}

            {/* Error message */}
            {error && (
              <div className="mt-4 p-3 rounded-lg bg-rose-50 border border-rose-200">
                <p className="text-sm text-rose-800">{error}</p>
              </div>
            )}
          </div>

          {/* Operations list - scrollable */}
          <div className="flex-1 overflow-y-auto p-6">
            <div className="space-y-3">
              {parseResult.operations.map((operation, index) => (
                <label
                  key={index}
                  className="flex items-start gap-3 p-4 rounded-xl border-2 border-neutral-light/40
                           hover:border-amber-200 hover:bg-amber-50/30 cursor-pointer transition-all duration-200
                           data-[selected=true]:border-amber-400 data-[selected=true]:bg-amber-50/50"
                  data-selected={selectedOperations.has(index)}
                >
                  <input
                    type="checkbox"
                    checked={selectedOperations.has(index)}
                    onChange={() => handleToggleOperation(index)}
                    className="mt-0.5 w-4 h-4 rounded border-neutral-dark/20 text-amber-500
                             focus:ring-2 focus:ring-amber-400/50 cursor-pointer"
                  />
                  <div className="flex-1 min-w-0">
                    <div className="flex items-center gap-2 mb-1">
                      {getOperationIcon(operation)}
                      <span className="text-sm font-medium text-neutral-dark">
                        {getOperationLabel(operation)}
                      </span>
                    </div>
                    <p className="text-sm text-neutral-dark/80 break-words">
                      {getOperationDescription(operation)}
                    </p>

                    {/* Additional details for Create operation */}
                    {operation.type === 'Create' && (
                      <div className="mt-2 space-y-1 text-xs text-neutral-dark/60">
                        {operation.data.description && (
                          <p>描述: {operation.data.description}</p>
                        )}
                        {operation.data.priority && (
                          <p>优先级: {operation.data.priority}</p>
                        )}
                        {operation.data.deadline && (
                          <p>截止: {new Date(operation.data.deadline).toLocaleString('zh-CN')}</p>
                        )}
                        {operation.data.tags && operation.data.tags.length > 0 && (
                          <p>标签: {operation.data.tags.join(', ')}</p>
                        )}
                      </div>
                    )}
                  </div>
                </label>
              ))}
            </div>
          </div>

          {/* Footer */}
          <div className="p-6 border-t border-neutral-light/40 flex items-center justify-between">
            <p className="text-sm text-neutral-dark/60">
              已选择 {selectedOperations.size} / {parseResult.operations.length} 项操作
            </p>
            <div className="flex gap-3">
              <button
                onClick={onCancel}
                className="px-4 py-2.5 rounded-lg border-2 border-neutral-light/40 text-neutral-dark
                         text-sm font-semibold transition-all duration-200
                         hover:bg-neutral-light/20"
              >
                取消
              </button>
              <button
                onClick={handleConfirm}
                disabled={isLoading || selectedOperations.size === 0}
                className="px-6 py-2.5 rounded-lg bg-gradient-to-r from-orange-400 to-rose-400
                         text-white text-sm font-semibold transition-all duration-200
                         hover:from-orange-500 hover:to-rose-500 hover:shadow-lg
                         disabled:opacity-50 disabled:cursor-not-allowed"
              >
                {isLoading ? '执行中...' : '确认执行'}
              </button>
            </div>
          </div>
        </div>
      </div>
    </>
  );
}

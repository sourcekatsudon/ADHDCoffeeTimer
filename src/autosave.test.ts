import { expect, it, vi } from 'vitest';

const { invoke } = vi.hoisted(() => ({ invoke: vi.fn().mockResolvedValue(undefined) }));
vi.mock('@tauri-apps/api/core', () => ({ isTauri: () => true, invoke }));

it('sends rapid preference changes as individual patches, without stale timer or other settings', async () => {
  const { updateSetting } = await import('./bridge');
  await Promise.all([
    updateSetting('opacity', 1.1),
    updateSetting('timerVisible', false),
    updateSetting('opacity', 1.2),
    updateSetting('timerVisible', true),
  ]);
  expect(invoke.mock.calls).toEqual([
    ['update_setting', { key: 'opacity', value: 1.1 }],
    ['update_setting', { key: 'timerVisible', value: false }],
    ['update_setting', { key: 'opacity', value: 1.2 }],
    ['update_setting', { key: 'timerVisible', value: true }],
  ]);
});

import { createSignal } from 'solid-js';

function Header() {
  const [userMenuOpen, setUserMenuOpen] = createSignal(false);

  return (
    <header aria-label="Page header" class="h-16 bg-[#12121a] border-b border-[#2a2a3a] flex items-center justify-between px-6">
      <div class="flex items-center gap-4">
        <h2 class="text-lg font-semibold">Dashboard</h2>
        <span class="px-2 py-1 bg-green-500/20 text-green-400 text-xs rounded-full">Live</span>
      </div>
      
      <div class="flex items-center gap-4">
        <button 
          title="Notifications"
          class="p-2 text-gray-400 hover:text-white transition-colors"
        >
          <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9"></path></svg>

        </button>
        
        <div class="relative">
          <button 
            title="User menu"
            onClick={() => setUserMenuOpen(!userMenuOpen())}
            class="flex items-center gap-2 p-2 rounded-lg hover:bg-[#2a2a3a] transition-colors"
          >
            <div class="w-8 h-8 rounded-full bg-[#00d4ff] flex items-center justify-center text-black font-semibold">
              U
            </div>
            <span class="text-sm text-gray-300">User</span>
          </button>
        </div>
      </div>
    </header>
  );
}

export default Header;

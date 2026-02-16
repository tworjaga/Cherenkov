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
          <span class="text-xl">🔔</span>
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

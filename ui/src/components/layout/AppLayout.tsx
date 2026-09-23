import React from 'react';
import { Outlet, NavLink } from 'react-router-dom';
import { LayoutDashboard, FolderTree, History, Shield, Settings as SettingsIcon } from 'lucide-react';
import styles from './AppLayout.module.css';

const AppLayout = () => {
  return (
    <div className={styles.appContainer}>
      {/* Sidebar Navigation */}
      <nav className={styles.sidebar}>
        <div className={styles.brand}>
          <div className={styles.logo}>CN</div>
          <span className={styles.brandName}>Organizer</span>
        </div>
        
        <div className={styles.navLinks}>
          <NavLink to="/dashboard" className={({isActive}) => `${styles.navItem} ${isActive ? styles.active : ''}`}>
            <LayoutDashboard size={20} />
            <span>Dashboard</span>
          </NavLink>
          <NavLink to="/organize" className={({isActive}) => `${styles.navItem} ${isActive ? styles.active : ''}`}>
            <FolderTree size={20} />
            <span>Organize</span>
          </NavLink>
          <NavLink to="/history" className={({isActive}) => `${styles.navItem} ${isActive ? styles.active : ''}`}>
            <History size={20} />
            <span>History</span>
          </NavLink>
          <NavLink to="/rules" className={({isActive}) => `${styles.navItem} ${isActive ? styles.active : ''}`}>
            <Shield size={20} />
            <span>Rules</span>
          </NavLink>
        </div>
        
        <div className={styles.navFooter}>
          <NavLink to="/settings" className={({isActive}) => `${styles.navItem} ${isActive ? styles.active : ''}`}>
            <SettingsIcon size={20} />
            <span>Settings</span>
          </NavLink>
        </div>
      </nav>

      {/* Main Content Area */}
      <main className={styles.mainContent}>
        <div className={styles.contentWrapper}>
          <Outlet />
        </div>
      </main>
    </div>
  );
};

export default AppLayout;

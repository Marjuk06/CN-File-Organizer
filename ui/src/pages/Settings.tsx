import React, { useState, useEffect } from 'react';
import { Settings as SettingsIcon, Save, Monitor, ShieldAlert } from 'lucide-react';
import { api } from '../lib/tauri';
import { Button } from '../components/ui/Button';
import { Card, CardHeader, CardTitle, CardContent, CardDescription } from '../components/ui/Card';
import styles from './Settings.module.css';

const Settings = () => {
  const [settings, setSettings] = useState<any>({
    auto_start: false,
    theme: 'dark',
    default_mode: 'Smart',
    language: 'en',
    ignore_hidden: true,
    safe_mode: true
  });
  const [isLoading, setIsLoading] = useState(true);
  const [isSaving, setIsSaving] = useState(false);

  useEffect(() => {
    loadSettings();
  }, []);

  const loadSettings = async () => {
    setIsLoading(true);
    try {
      const data = await api.getSettings();
      setSettings(data);
    } catch (err) {
      console.error('Failed to load settings', err);
    } finally {
      setIsLoading(false);
    }
  };

  const handleSave = async () => {
    setIsSaving(true);
    try {
      await api.saveSettings(settings);
      alert('Settings saved successfully');
    } catch (err) {
      alert('Failed to save settings: ' + err);
    } finally {
      setIsSaving(false);
    }
  };

  const handleChange = (key: string, value: any) => {
    setSettings({ ...settings, [key]: value });
  };

  return (
    <div className={styles.container}>
      <div className={styles.header}>
        <div className={styles.titleArea}>
          <SettingsIcon size={32} className={styles.icon} />
          <div>
            <h1 className={styles.title}>Settings</h1>
            <p className={styles.subtitle}>Configure application behavior and preferences.</p>
          </div>
        </div>
        <div className={styles.actions}>
          <Button variant="primary" onClick={handleSave} disabled={isSaving || isLoading}>
            <Save size={18} style={{marginRight: '8px'}} /> Save Preferences
          </Button>
        </div>
      </div>

      <div className={styles.settingsGrid}>
        <Card className="glass-panel">
          <CardHeader>
            <CardTitle className={styles.cardTitle}>
              <Monitor size={20} /> Appearance
            </CardTitle>
            <CardDescription>Customize the look and feel of the application.</CardDescription>
          </CardHeader>
          <CardContent className={styles.formGroup}>
            <div className={styles.settingRow}>
              <div>
                <label className={styles.settingLabel}>Theme</label>
                <p className={styles.settingDesc}>Select your preferred color theme.</p>
              </div>
              <select 
                className={styles.select} 
                value={settings.theme} 
                onChange={(e) => handleChange('theme', e.target.value)}
              >
                <option value="light">Light</option>
                <option value="dark">Dark</option>
                <option value="system">System Default</option>
              </select>
            </div>
            
            <div className={styles.settingRow}>
              <div>
                <label className={styles.settingLabel}>Language</label>
                <p className={styles.settingDesc}>Application interface language.</p>
              </div>
              <select 
                className={styles.select} 
                value={settings.language} 
                onChange={(e) => handleChange('language', e.target.value)}
              >
                <option value="en">English</option>
                <option value="bn">Bengali</option>
                <option value="fr">French</option>
              </select>
            </div>
          </CardContent>
        </Card>

        <Card className="glass-panel">
          <CardHeader>
            <CardTitle className={styles.cardTitle}>
              <ShieldAlert size={20} /> Organization Safety
            </CardTitle>
            <CardDescription>Configure how files are handled during organization.</CardDescription>
          </CardHeader>
          <CardContent className={styles.formGroup}>
            <div className={styles.settingRow}>
              <div>
                <label className={styles.settingLabel}>Safe Mode</label>
                <p className={styles.settingDesc}>Require explicit confirmation before modifying any files outside of the selected root.</p>
              </div>
              <label className={styles.toggle}>
                <input 
                  type="checkbox" 
                  checked={settings.safe_mode} 
                  onChange={(e) => handleChange('safe_mode', e.target.checked)} 
                />
                <span className={styles.slider}></span>
              </label>
            </div>
            
            <div className={styles.settingRow}>
              <div>
                <label className={styles.settingLabel}>Ignore Hidden Files</label>
                <p className={styles.settingDesc}>Skip files starting with a dot (e.g. .git, .DS_Store).</p>
              </div>
              <label className={styles.toggle}>
                <input 
                  type="checkbox" 
                  checked={settings.ignore_hidden} 
                  onChange={(e) => handleChange('ignore_hidden', e.target.checked)} 
                />
                <span className={styles.slider}></span>
              </label>
            </div>
          </CardContent>
        </Card>
      </div>
    </div>
  );
};

export default Settings;

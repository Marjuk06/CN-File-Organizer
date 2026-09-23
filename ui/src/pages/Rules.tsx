import React, { useState, useEffect } from 'react';
import { Shield, Plus, Trash2, Edit2, Save } from 'lucide-react';
import { api } from '../lib/tauri';
import { Button } from '../components/ui/Button';
import { Card, CardContent } from '../components/ui/Card';
import styles from './Rules.module.css';

const Rules = () => {
  const [rules, setRules] = useState<any[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [isSaving, setIsSaving] = useState(false);

  useEffect(() => {
    loadRules();
  }, []);

  const loadRules = async () => {
    setIsLoading(true);
    try {
      const data = await api.getRules();
      setRules(data);
    } catch (err) {
      console.error('Failed to load rules', err);
    } finally {
      setIsLoading(false);
    }
  };

  const handleSave = async () => {
    setIsSaving(true);
    try {
      await api.saveRules(rules);
      alert('Rules saved successfully');
    } catch (err) {
      alert('Failed to save rules: ' + err);
    } finally {
      setIsSaving(false);
    }
  };

  const handleDelete = (index: number) => {
    const newRules = [...rules];
    newRules.splice(index, 1);
    setRules(newRules);
  };

  return (
    <div className={styles.container}>
      <div className={styles.header}>
        <div className={styles.titleArea}>
          <Shield size={32} className={styles.icon} />
          <div>
            <h1 className={styles.title}>Custom Rules</h1>
            <p className={styles.subtitle}>Define specific matching patterns and destinations.</p>
          </div>
        </div>
        <div className={styles.actions}>
          <Button variant="primary" onClick={handleSave} disabled={isSaving}>
            <Save size={18} style={{marginRight: '8px'}} /> Save Changes
          </Button>
        </div>
      </div>

      <div className={styles.rulesList}>
        {isLoading ? (
          <div className={styles.emptyState}>Loading rules...</div>
        ) : rules.length === 0 ? (
          <Card className="glass-panel">
            <CardContent className={styles.emptyState}>
              <Shield size={48} className={styles.emptyIcon} />
              <h3>No Custom Rules</h3>
              <p>You haven't defined any custom rules yet. Using default organization behaviors.</p>
              <Button variant="secondary" className={styles.addRuleBtn}>
                <Plus size={18} style={{marginRight: '6px'}} /> Add Rule
              </Button>
            </CardContent>
          </Card>
        ) : (
          rules.map((rule, index) => (
            <Card key={rule.id} className="glass-panel">
              <CardContent className={styles.ruleCard}>
                <div className={styles.ruleDetails}>
                  <h3>{rule.name}</h3>
                  <div className={styles.rulePatterns}>
                    {rule.patterns.map((p: string) => (
                      <span key={p} className={styles.patternBadge}>{p}</span>
                    ))}
                  </div>
                  <div className={styles.ruleDest}>
                    <span className={styles.destLabel}>Destination:</span> {rule.target_folder}
                  </div>
                </div>
                <div className={styles.ruleActions}>
                  <Button variant="ghost" size="sm"><Edit2 size={16} /></Button>
                  <Button variant="danger" size="sm" onClick={() => handleDelete(index)}><Trash2 size={16} /></Button>
                </div>
              </CardContent>
            </Card>
          ))
        )}
      </div>
    </div>
  );
};

export default Rules;

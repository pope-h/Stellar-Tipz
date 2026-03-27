import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import Input from '@/components/ui/Input';
import Textarea from '@/components/ui/Textarea';
import Button from '@/components/ui/Button';
import TransactionStatus from '@/components/shared/TransactionStatus';
import { useContract } from '@/hooks';
import { useToastStore } from '@/store/toastStore';
import type { Profile } from '@/types/contract';
import type { ProfileFormData } from '@/types/profile';

type TxStatus = 'idle' | 'signing' | 'submitting' | 'confirming' | 'success' | 'error';

interface FormErrors {
  displayName?: string;
  bio?: string;
  imageUrl?: string;
  xHandle?: string;
}

function validate(data: ProfileFormData): FormErrors {
  const errors: FormErrors = {};

  if (!data.displayName.trim() || data.displayName.length > 64) {
    errors.displayName = 'Display name is required and must be 1–64 characters.';
  }

  if (data.bio.length > 280) {
    errors.bio = 'Bio must be 280 characters or fewer.';
  }

  return errors;
}

interface EditProfileFormProps {
  profile: Profile;
}

const EditProfileForm: React.FC<EditProfileFormProps> = ({ profile }) => {
  const [form, setForm] = useState<ProfileFormData>({
    username: profile.username,
    displayName: profile.displayName,
    bio: profile.bio,
    imageUrl: profile.imageUrl,
    xHandle: profile.xHandle,
  });
  const [errors, setErrors] = useState<FormErrors>({});
  const [txStatus, setTxStatus] = useState<TxStatus>('idle');
  const [txHash, setTxHash] = useState<string | undefined>(undefined);
  const [txError, setTxError] = useState<string | undefined>(undefined);

  const { updateProfile } = useContract();
  const { addToast } = useToastStore();
  const navigate = useNavigate();

  const handleChange =
    (field: keyof ProfileFormData) =>
    (e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => {
      setForm((prev) => ({ ...prev, [field]: e.target.value }));
      if (errors[field as keyof FormErrors]) {
        setErrors((prev) => ({ ...prev, [field]: undefined }));
      }
    };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    const validationErrors = validate(form);
    if (Object.keys(validationErrors).length > 0) {
      setErrors(validationErrors);
      return;
    }

    try {
      setTxStatus('signing');
      setTxError(undefined);
      setTxHash(undefined);

      const data: Partial<ProfileFormData> = {
        displayName: form.displayName.trim(),
        bio: form.bio.trim(),
        imageUrl: form.imageUrl.trim(),
        xHandle: form.xHandle.trim().replace(/^@/, ''),
      };

      setTxStatus('submitting');
      const hash = await updateProfile(data);

      setTxStatus('confirming');
      setTxHash(hash);

      setTxStatus('success');
      addToast({ message: 'Profile updated successfully!', type: 'success', duration: 5000 });

      setTimeout(() => navigate('/profile'), 1500);
    } catch (err) {
      setTxStatus('error');
      setTxError(err instanceof Error ? err.message : 'Update failed. Please try again.');
    }
  };

  const isSubmitting = ['signing', 'submitting', 'confirming'].includes(txStatus);

  return (
    <form onSubmit={handleSubmit} className="space-y-6 max-w-lg mx-auto">
      {/* Username (read-only) */}
      <div>
        <Input
          label="Username"
          value={form.username}
          disabled
        />
        <p className="mt-1 text-xs text-gray-500">
          Username cannot be changed after registration.
        </p>
      </div>

      {/* Display Name */}
      <Input
        label="Display Name"
        placeholder="Your Name"
        value={form.displayName}
        onChange={handleChange('displayName')}
        error={errors.displayName}
        disabled={isSubmitting}
        maxLength={64}
        required
      />

      {/* Bio */}
      <Textarea
        label="Bio"
        placeholder="Tell supporters about yourself…"
        value={form.bio}
        onChange={handleChange('bio')}
        error={errors.bio}
        disabled={isSubmitting}
        maxLength={280}
        rows={4}
      />

      {/* X Handle */}
      <Input
        label="X (Twitter) Handle (optional)"
        placeholder="@yourhandle"
        value={form.xHandle}
        onChange={handleChange('xHandle')}
        error={errors.xHandle}
        disabled={isSubmitting}
      />

      {/* Image URL */}
      <Input
        label="Profile Image URL (optional)"
        placeholder="https://example.com/avatar.png"
        type="url"
        value={form.imageUrl}
        onChange={handleChange('imageUrl')}
        error={errors.imageUrl}
        disabled={isSubmitting}
      />

      {/* Transaction status */}
      {txStatus !== 'idle' && (
        <TransactionStatus
          status={txStatus}
          txHash={txHash}
          errorMessage={txError}
          onRetry={() => setTxStatus('idle')}
        />
      )}

      <Button
        type="submit"
        variant="primary"
        size="lg"
        disabled={isSubmitting || txStatus === 'success'}
        className="w-full"
      >
        {isSubmitting ? 'Updating…' : 'Save Changes'}
      </Button>
    </form>
  );
};

export default EditProfileForm;

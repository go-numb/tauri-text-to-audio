import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import './App.css';

import { Button, Flex } from 'antd';

import 'regenerator-runtime/runtime';
import SpeechRecognition, { useSpeechRecognition } from 'react-speech-recognition';

const stopWords = ['。', '、', '？', '！', '…', 'ー', '〜', ',', '.', '?', '!', '-', '~'];

const includeStopWords = (text: string) => {
  for (const stopWord of stopWords) {
    if (text.includes(stopWord)) {
      return true;
    }
  }
  return false;
}

function App() {
  const [checked, setChecked] = useState(true);
  const {
    transcript,
    listening,
    resetTranscript,
    browserSupportsSpeechRecognition
  } = useSpeechRecognition();

  if (!browserSupportsSpeechRecognition) {
    setChecked(false);
  }

  const [lang, setLang] = useState('ja');

  const [isProcessing, setIsProcessing] = useState(false);
  const [_, setError] = useState<String>('');

  // transcript の変更を監視するeffect
  useEffect(() => {
    let isMounted = true;

    const handleTranscript = async () => {
      if (!transcript || isProcessing) return;

      if (includeStopWords(transcript)) {
        const text = transcript.trim();
        if (text && isMounted) {
          resetTranscript();
          console.log('transcript:', text);

          await process_audio(text);
        }
      }
    };

    handleTranscript();

    // クリーンアップ関数
    return () => {
      isMounted = false;
    };
  }, [transcript, isProcessing]);


  // 音声処理用の独立した関数
  const process_audio = async (text: string) => {
    try {
      setIsProcessing(true);
      await to_audio(text);
    } catch (err) {
      setError(`音声処理エラー: ${err}`);
      console.error('音声処理エラー:', err);
    } finally {
      setIsProcessing(false);
    }
  };

  const to_audio = async (text: string) => {
    const response = await invoke('to_audio', { text });
    console.log(response);
  }

  const switch_lang = () => {
    setLang(lang === 'ja' ? 'en' : 'ja');
  }

  return (
    <main className="container">
      <h1>Audio to text, and change voice with TTL.</h1>

      {
        checked ? (
          <>
            <p>Microphone: {listening ? 'on' : 'off'}</p>
            <Flex vertical gap={20}>

              <Flex vertical={false} gap={20} justify='center'>
                <Button type='primary' onClick={() => {
                  SpeechRecognition.startListening({ language: lang, continuous: true });
                }}>Start</Button>
                <Button onClick={SpeechRecognition.stopListening}>Stop</Button>
                <Button onClick={resetTranscript}>Reset</Button>
                <Button onClick={switch_lang}>
                  {lang === 'ja' ? '日本語' : 'English'}
                </Button>
              </Flex>
            </Flex>
            <p>{transcript}</p>
          </>
        ) : (
          <p>Speech Recognition is not supported in your browser</p>
        )
      }

    </main>
  );
}

export default App;
